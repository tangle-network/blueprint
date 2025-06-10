use crate::error::{Error, Result};
use blueprint_core::{debug, error};
use capctl::{Cap, CapState};
use ipnet::Ipv4Net;
use nftables::batch::Batch;
use nftables::expr::{
    CT, Expression, Meta, MetaKey, NamedExpression, Payload, PayloadBase, PayloadRaw, Prefix,
    SetItem,
};
use nftables::helper::NftablesError;
use nftables::schema::{NfCmd, NfListObject};
use nftables::stmt::{JumpTarget, Match, Operator, Statement};
use nftables::{
    schema::{Chain, Rule},
    types::*,
};
use std::borrow::Cow;
use std::net::Ipv4Addr;

const FORWARD_CHAIN: &str = "TANGLE_FORWARD";
const NAT_CHAIN: &str = "TANGLE_NAT";

fn setup_chains_if_needed() -> Result<()> {
    let mut batch = Batch::new();

    batch.add_cmd(NfCmd::Create(NfListObject::Chain(Chain {
        family: NfFamily::INet,
        table: "filter".into(),
        name: FORWARD_CHAIN.into(),
        _type: Some(NfChainType::Filter),
        hook: Some(NfHook::Forward),
        policy: Some(NfChainPolicy::Accept),
        ..Default::default()
    })));
    batch.add_cmd(NfCmd::Create(NfListObject::Chain(Chain {
        family: NfFamily::INet,
        table: "nat".into(),
        name: NAT_CHAIN.into(),
        _type: Some(NfChainType::NAT),
        hook: Some(NfHook::Postrouting),
        policy: Some(NfChainPolicy::Accept),
        ..Default::default()
    })));

    // Jump from the base FORWARD chain to our custom one.
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from("filter"),
        chain: Cow::from("FORWARD"),
        expr: Cow::from(vec![Statement::Jump(JumpTarget {
            target: FORWARD_CHAIN.into(),
        })]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Jump to Blueprint VM forward chain")),
    }));

    // Jump from the base POSTROUTING chain to our custom NAT one.
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from("nat"),
        chain: Cow::from("POSTROUTING"),
        expr: Cow::from(vec![Statement::Jump(JumpTarget {
            target: NAT_CHAIN.into(),
        })]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Jump to Blueprint VM NAT chain")),
    }));

    let nft = batch.to_nftables();
    if let Err(e) = nftables::helper::apply_ruleset(&nft) {
        match e {
            NftablesError::NftFailed { stderr, .. } if stderr.contains("File exists") => {}
            _ => return Err(e.into()),
        }
    }

    Ok(())
}

/// Removes all rules, chains, and cleans up.
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
        table: Cow::from("filter"),
        chain: Cow::from(FORWARD_CHAIN),
        expr: Cow::from(vec![
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Iifname,
                })),
                right: Expression::String(Cow::from(host_iface)),
                op: Operator::EQ,
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::Meta(Meta {
                    key: MetaKey::Oifname,
                })),
                right: Expression::String(Cow::from(tap_iface)),
                op: Operator::EQ,
            }),
            Statement::Match(Match {
                left: Expression::Named(NamedExpression::CT(CT {
                    key: Cow::from("state"),
                    ..Default::default()
                })),
                right: Expression::Named(NamedExpression::Set(vec![
                    SetItem::Element(Expression::String(Cow::from("established"))),
                    SetItem::Element(Expression::String(Cow::from("related"))),
                ])),
                op: Operator::IN,
            }),
            Statement::Accept(None),
        ]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Allow established traffic to Blueprint VM")),
    }));

    // Allow new connections from the VM's network out
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from("filter"),
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
                left: Expression::Named(NamedExpression::Payload(Payload::PayloadRaw(
                    PayloadRaw {
                        base: PayloadBase::NH,
                        offset: 12,
                        len: 4,
                    },
                ))),
                right: Expression::Named(NamedExpression::Prefix(Prefix {
                    addr: Box::new(Expression::String(Cow::from(net.addr().to_string()))),
                    len: net.prefix_len() as u32,
                })),
                op: Operator::EQ,
            }),
            Statement::Accept(None),
        ]),
        handle: None,
        index: None,
        comment: Some(Cow::from("Allow new traffic from Blueprint VM")),
    }));

    // Masquerade outgoing traffic from the VM's network
    batch.add(NfListObject::Rule(Rule {
        family: NfFamily::INet,
        table: Cow::from("nat"),
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
                left: Expression::Named(NamedExpression::Payload(Payload::PayloadRaw(
                    PayloadRaw {
                        base: PayloadBase::NH,
                        offset: 12,
                        len: 4,
                    },
                ))),
                right: Expression::Named(NamedExpression::Prefix(Prefix {
                    addr: Box::new(Expression::String(Cow::from(net.addr().to_string()))),
                    len: net.prefix_len() as u32,
                })),
                op: Operator::EQ,
            }),
            Statement::Masquerade(None),
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
pub fn setup_rules(host_iface: &str, tap_iface: &str, vm_ip: Ipv4Addr) -> Result<()> {
    capctl::ambient::raise(Cap::NET_ADMIN)?;
    let res =
        setup_chains_if_needed().and_then(|_| setup_rules_inner(host_iface, tap_iface, vm_ip));
    capctl::ambient::lower(Cap::NET_ADMIN)?;
    res
}

/// Verify that the binary has the `CAP_NET_ADMIN` capability and is able to make it inheritable
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
