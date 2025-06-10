use crate::error::{Error, Result};
use blueprint_core::{debug, error};
use capctl::{Cap, CapState};
use ipnet::Ipv4Net;
use nftables::batch::Batch;
use nftables::expr::{
    CT, Expression, Meta, MetaKey, NamedExpression, Payload, PayloadField, Prefix, SetItem,
};
use nftables::schema::{Chain, Rule};
use nftables::schema::{NfCmd, NfListObject, Table};
use nftables::stmt::{Accept, Match, NAT, Operator, Statement};
use nftables::types::{NfChainPolicy, NfChainType, NfFamily, NfHook};
use std::borrow::Cow;
use std::net::Ipv4Addr;

const TANGLE_ROUTER_TABLE: &str = "tangle_router";
const FORWARD_CHAIN: &str = "TANGLE_FORWARD";
const PRIORITY_FILTER: i32 = 0;

const NAT_CHAIN: &str = "TANGLE_NAT";
const PRIORITY_SRCNAT: i32 = 100;

fn setup_chains_if_needed() -> Result<()> {
    let mut batch = Batch::new();

    batch.add_cmd(NfCmd::Add(NfListObject::Table(Table {
        family: NfFamily::INet,
        name: TANGLE_ROUTER_TABLE.into(),
        ..Default::default()
    })));

    batch.add_cmd(NfCmd::Add(NfListObject::Chain(Chain {
        family: NfFamily::INet,
        table: TANGLE_ROUTER_TABLE.into(),
        name: FORWARD_CHAIN.into(),
        _type: Some(NfChainType::Filter),
        hook: Some(NfHook::Forward),
        policy: Some(NfChainPolicy::Accept),
        prio: Some(PRIORITY_FILTER),
        ..Default::default()
    })));

    batch.add_cmd(NfCmd::Add(NfListObject::Chain(Chain {
        family: NfFamily::INet,
        table: TANGLE_ROUTER_TABLE.into(),
        name: NAT_CHAIN.into(),
        _type: Some(NfChainType::NAT),
        hook: Some(NfHook::Postrouting),
        policy: Some(NfChainPolicy::Accept),
        prio: Some(PRIORITY_SRCNAT),
        ..Default::default()
    })));

    let nft = batch.to_nftables();
    nftables::helper::apply_ruleset(&nft)?;

    Ok(())
}

/// Removes all rules, chains, and cleans up.
///
/// # Errors
///
/// TODO
pub fn cleanup_firewall(host_iface: &str) -> Result<()> {
    // TODO: Actually do cleanup

    debug!("Removed custom chains and rules on interface {host_iface}");
    Ok(())
}

fn setup_rules_inner(host_iface: &str, tap_iface: &str, vm_ip: Ipv4Addr) -> Result<()> {
    let net = Ipv4Net::new(vm_ip, 24).unwrap();
    let mut batch = Batch::new();

    // Allow established and related connections back to the VM
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from(TANGLE_ROUTER_TABLE),
        chain: Cow::from(FORWARD_CHAIN),
        expr: Cow::from(vec![
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Iifname,
                })),
                op: Operator::EQ,
                right: Expression::String(Cow::from(host_iface)),
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Oifname,
                })),
                op: Operator::EQ,
                right: Expression::String(Cow::from(tap_iface)),
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::CT(CT {
                    key: Cow::from("state"),
                    ..Default::default()
                })),
                op: Operator::IN,
                right: Expression::Named(NamedExpression::Set(vec![
                    SetItem::Element(Expression::String(Cow::from("established"))),
                    SetItem::Element(Expression::String(Cow::from("related"))),
                ])),
            }),
            Statement::Accept(Some(Accept {})),
        ]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Allow established traffic to Blueprint VM")),
    }));

    // Allow new connections from the VM's network out
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from(TANGLE_ROUTER_TABLE),
        chain: Cow::from(FORWARD_CHAIN),
        expr: Cow::from(vec![
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Iifname,
                })),
                right: Expression::String(Cow::from(tap_iface)),
                op: Operator::EQ,
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Oifname,
                })),
                right: Expression::String(Cow::from(host_iface)),
                op: Operator::EQ,
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Payload(Payload::PayloadField(
                    PayloadField {
                        protocol: "ip".into(),
                        field: "saddr".into(),
                    },
                ))),
                right: Expression::Named(NamedExpression::Prefix(Prefix {
                    addr: Box::new(Expression::String(Cow::from(net.network().to_string()))),
                    len: u32::from(net.prefix_len()),
                })),
                op: Operator::EQ,
            }),
            Statement::Accept(Some(Accept {})),
        ]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Allow new traffic from Blueprint VM")),
    }));

    // Masquerade outgoing traffic from the VM's network
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from(TANGLE_ROUTER_TABLE),
        chain: Cow::from(NAT_CHAIN),
        expr: Cow::from(vec![
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Oifname,
                })),
                right: Expression::String(Cow::from(host_iface)),
                op: Operator::EQ,
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Payload(Payload::PayloadField(
                    PayloadField {
                        protocol: "ip".into(),
                        field: "saddr".into(),
                    },
                ))),
                op: Operator::EQ,
                right: Expression::Named(NamedExpression::Prefix(Prefix {
                    addr: Box::new(Expression::String(Cow::from(net.network().to_string()))),
                    len: u32::from(net.prefix_len()),
                })),
            }),
            Statement::Masquerade(Some(NAT {
                addr: None,
                family: None,
                port: None,
                flags: None,
            })),
        ]),
        handle: None,
        index: None,
        comment: Some(Cow::from("NAT traffic from Blueprint VM")),
    }));

    let nft = batch.to_nftables();
    nftables::helper::apply_ruleset(&nft)?;

    Ok(())
}

/// Setup nftables rules for the VM networking
pub(crate) fn setup_rules(host_iface: &str, tap_iface: &str, vm_ip: Ipv4Addr) -> Result<()> {
    capctl::ambient::raise(Cap::NET_ADMIN)?;
    let res =
        setup_chains_if_needed().and_then(|()| setup_rules_inner(host_iface, tap_iface, vm_ip));
    capctl::ambient::lower(Cap::NET_ADMIN)?;
    res
}

/// Verify that the binary has the `CAP_NET_ADMIN` capability and is able to make it inheritable
///
/// # Errors
///
/// * Unable to check the capabilities
/// * The process doesn't have the `CAP_NET_ADMIN` capability
pub fn check_net_admin_capability() -> Result<()> {
    let Ok(mut state) = CapState::get_current() else {
        error!("Unable to get the current thread's capabilities");
        return Err(Error::Other(String::from(
            "Unable to get the current thread's capabilities",
        )));
    };

    if state.effective.has(Cap::NET_ADMIN) {
        state.inheritable.add(Cap::NET_ADMIN);
        state.set_current()?;
        Ok(())
    } else {
        error!("This program requires the CAP_NET_ADMIN capability");
        Err(Error::Other(String::from(
            "Binary is missing the CAP_NET_ADMIN capability",
        )))
    }
}
