use blueprint_chain_setup::tangle::InputValue;
use blueprint_tangle_extra::serde::{BoundedVec, Field, from_field, new_bounded_string};
use color_eyre::Result;
use color_eyre::eyre::bail;
use dialoguer::console::style;
use serde_json;
use std::str::FromStr;
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::FieldType;

pub(crate) fn print_job_results(result_types: &[FieldType], i: usize, field: Field<AccountId32>) {
    let expected_type = result_types[i].clone();
    let print_output = match expected_type {
        FieldType::Void => "void".to_string(),
        FieldType::Bool => {
            let output: bool = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Uint8 => {
            let output: u8 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Int8 => {
            let output: i8 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Uint16 => {
            let output: u16 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Int16 => {
            let output: i16 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Uint32 => {
            let output: u32 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Int32 => {
            let output: i32 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Uint64 => {
            let output: u64 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Int64 => {
            let output: i64 = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::String => {
            let output: String = from_field(field).unwrap();
            output.to_string()
        }
        FieldType::Optional(_field_type) => {
            let output: Option<FieldType> = from_field(field.clone()).unwrap();
            if output.is_some() {
                let output: FieldType = output.unwrap();
                print_job_results(&[output], 0, field);
                "Some".to_string()
            } else {
                "None".to_string()
            }
        }
        FieldType::Array(_, _field_type) => {
            let output: Vec<FieldType> = from_field(field.clone()).unwrap();
            for (i, inner_type) in output.iter().enumerate() {
                print_job_results(&[inner_type.clone()], i, field.clone());
            }
            "Array".to_string()
        }
        FieldType::List(_field_type) => {
            let output: BoundedVec<FieldType> = from_field(field.clone()).unwrap();
            for (i, inner_type) in output.0.iter().enumerate() {
                print_job_results(&[inner_type.clone()], i, field.clone());
            }
            "List".to_string()
        }
        FieldType::Struct(_bounded_vec) => {
            let output: BoundedVec<FieldType> = from_field(field.clone()).unwrap();
            for (i, inner_type) in output.0.iter().enumerate() {
                print_job_results(&[inner_type.clone()], i, field.clone());
            }
            "Struct".to_string()
        }
        FieldType::AccountId => {
            let output: AccountId32 = from_field(field).unwrap();
            output.to_string()
        }
    };
    println!(
        "{}: {}",
        style(format!("Output {}", i + 1)).green().bold(),
        style(format!("{:?}", print_output)).green()
    );
}

/// Load job arguments from a JSON file
///
/// # Arguments
///
/// * `file_path` - Path to the JSON file
/// * `param_types` - Types of parameters expected
///
/// # Returns
///
/// A vector of input values parsed from the file.
///
/// # Errors
///
/// Returns an error if:
/// * File not found
/// * File content is not valid JSON
/// * JSON is not an array
/// * Number of arguments doesn't match expected parameters
/// * Arguments don't match expected types
pub(crate) fn load_job_args_from_file(
    file_path: &str,
    param_types: &[FieldType],
) -> Result<Vec<InputValue>> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Parameters file not found: {}",
            file_path
        ));
    }

    let content = fs::read_to_string(path)?;
    let json_values: serde_json::Value = serde_json::from_str(&content)?;

    if !json_values.is_array() {
        return Err(color_eyre::eyre::eyre!(
            "Job arguments must be provided as a JSON array"
        ));
    }

    let args = json_values.as_array().unwrap();

    if args.len() != param_types.len() {
        return Err(color_eyre::eyre::eyre!(
            "Expected {} arguments but got {}",
            param_types.len(),
            args.len()
        ));
    }

    // Parse each argument according to the expected parameter type
    let input_values = json_to_input_value(param_types, args, 0)?;

    Ok(input_values)
}

/// Prompt the user for job parameters based on the parameter types
pub(crate) fn prompt_for_job_params(param_types: &[FieldType]) -> Result<Vec<InputValue>> {
    use dialoguer::Input;

    let mut args = Vec::new();

    for (i, param_type) in param_types.iter().enumerate() {
        println!("Parameter {}: {:?}", i + 1, param_type);

        match param_type {
            FieldType::Uint8 => {
                let value: u8 = Input::new()
                    .with_prompt(format!("Enter u8 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Uint8(value));
            }
            FieldType::Uint16 => {
                let value: u16 = Input::new()
                    .with_prompt(format!("Enter u16 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Uint16(value));
            }
            FieldType::Uint32 => {
                let value: u32 = Input::new()
                    .with_prompt(format!("Enter u32 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Uint32(value));
            }
            FieldType::Uint64 => {
                let value: u64 = Input::new()
                    .with_prompt(format!("Enter u64 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Uint64(value));
            }
            FieldType::Int8 => {
                let value: i8 = Input::new()
                    .with_prompt(format!("Enter i8 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Int8(value));
            }
            FieldType::Int16 => {
                let value: i16 = Input::new()
                    .with_prompt(format!("Enter i16 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Int16(value));
            }
            FieldType::Int32 => {
                let value: i32 = Input::new()
                    .with_prompt(format!("Enter i32 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Int32(value));
            }
            FieldType::Int64 => {
                let value: i64 = Input::new()
                    .with_prompt(format!("Enter i64 value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::Int64(value));
            }
            FieldType::Bool => {
                let value: bool = Input::new()
                    .with_prompt(format!(
                        "Enter boolean value (true/false) for parameter {}",
                        i + 1
                    ))
                    .interact()?;
                args.push(InputValue::Bool(value));
            }
            FieldType::String => {
                let value: String = Input::new()
                    .with_prompt(format!("Enter string value for parameter {}", i + 1))
                    .interact()?;
                args.push(InputValue::String(new_bounded_string(value)));
            }
            FieldType::Void => {
                println!("Void parameter, no input required");
            }
            FieldType::Optional(field_type) => {
                use dialoguer::Confirm;

                let include_value = Confirm::new()
                    .with_prompt(format!("Include a value for optional parameter {}?", i + 1))
                    .default(false)
                    .interact()?;

                if include_value {
                    // Recursively prompt for the inner type
                    let inner_values = prompt_for_job_params(&[*field_type.clone()])?;
                    if let Some(inner_value) = inner_values.first() {
                        args.push(InputValue::Optional(
                            (**field_type).clone(),
                            Box::new(Some(inner_value.clone())),
                        ));
                    }
                } else {
                    args.push(InputValue::Optional((**field_type).clone(), Box::new(None)));
                }
            }
            FieldType::Array(size, field_type) => {
                println!("Enter {} values for array parameter {}", size, i + 1);
                let mut array_values = Vec::new();

                for j in 0..*size {
                    println!("Array element {} of {}", j + 1, size);
                    let inner_values = prompt_for_job_params(&[*field_type.clone()])?;
                    if let Some(inner_value) = inner_values.first() {
                        array_values.push(inner_value.clone());
                    }
                }

                let values = BoundedVec(array_values);
                args.push(InputValue::Array(*field_type.clone(), values));
            }
            FieldType::List(field_type) => {
                use dialoguer::Input;

                let count: usize = Input::new()
                    .with_prompt(format!("How many elements for list parameter {}?", i + 1))
                    .default(0)
                    .interact()?;

                let mut list_values = Vec::new();

                for j in 0..count {
                    println!("List element {} of {}", j + 1, count);
                    let inner_values = prompt_for_job_params(&[*field_type.clone()])?;
                    if let Some(inner_value) = inner_values.first() {
                        list_values.push(inner_value.clone());
                    }
                }

                let values = BoundedVec(list_values);
                args.push(InputValue::List(*field_type.clone(), values));
            }
            FieldType::Struct(_bounded_vec) => {
                todo!();
            }
            FieldType::AccountId => {
                let value: String = Input::new()
                    .with_prompt(format!(
                        "Enter AccountId for parameter {} (SS58 format)",
                        i + 1
                    ))
                    .interact()?;

                // Parse the account ID from the string
                match AccountId32::from_str(&value) {
                    Ok(account_id) => args.push(InputValue::AccountId(account_id)),
                    Err(_) => return Err(color_eyre::eyre::eyre!("Invalid AccountId format")),
                }
            }
        }
    }

    Ok(args)
}

/// build `InputValue` from JSON values
pub(crate) fn json_to_input_value(
    types: &[FieldType],
    values: &[serde_json::Value],
    depth: usize,
) -> Result<Vec<InputValue>> {
    let mut args = Vec::new();

    for (i, value) in values.iter().enumerate() {
        let field_type = &types[i];
        match field_type {
            FieldType::Uint8 => {
                let v = value
                    .as_u64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected u8 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for u8 (depth: {depth})")
                    })?;
                args.push(InputValue::Uint8(v));
            }
            FieldType::Uint16 => {
                let v = value
                    .as_u64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected u16 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for u16 (depth: {depth})")
                    })?;
                args.push(InputValue::Uint16(v));
            }
            FieldType::Uint32 => {
                let v = value
                    .as_u64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected u32 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for u32 (depth: {depth})")
                    })?;
                args.push(InputValue::Uint32(v));
            }
            FieldType::Uint64 => {
                let v = value.as_u64().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected u64 value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;
                args.push(InputValue::Uint64(v));
            }
            FieldType::Int8 => {
                let v = value
                    .as_i64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected i8 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for i8 (depth: {depth})")
                    })?;
                args.push(InputValue::Int8(v));
            }
            FieldType::Int16 => {
                let v = value
                    .as_i64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected i16 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for i16 (depth: {depth})")
                    })?;
                args.push(InputValue::Int16(v));
            }
            FieldType::Int32 => {
                let v = value
                    .as_i64()
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!(
                            "Expected i32 value for parameter {} (depth: {depth})",
                            i + 1
                        )
                    })?
                    .try_into()
                    .map_err(|_| {
                        color_eyre::eyre::eyre!("Value out of range for i32 (depth: {depth})")
                    })?;
                args.push(InputValue::Int32(v));
            }
            FieldType::Int64 => {
                let v = value.as_i64().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected i64 value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;
                args.push(InputValue::Int64(v));
            }
            FieldType::Bool => {
                let v = value.as_bool().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected bool value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;
                args.push(InputValue::Bool(v));
            }
            FieldType::String => {
                let v = value.as_str().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected string value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;
                args.push(InputValue::String(new_bounded_string(v)));
            }
            FieldType::Void => {
                // Void parameter, no input required
            }
            FieldType::Optional(field_type) => {
                if value.is_null() {
                    args.push(InputValue::Optional((**field_type).clone(), Box::new(None)));
                } else {
                    // Recursively prompt for the inner type.
                    let inner_values =
                        json_to_input_value(&[*field_type.clone()], &[value.clone()], depth + 1)?;
                    for val in inner_values {
                        args.push(InputValue::Optional(
                            (**field_type).clone(),
                            Box::new(Some(val)),
                        ));
                    }
                }
            }
            FieldType::Array(size, field_type) => {
                let mut array_values = Vec::new();
                let v = value.as_array().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected array value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;

                if v.len() as u64 != *size {
                    bail!(
                        "Expected {} elements in array for parameter {} (depth {}), but got {}",
                        size,
                        i + 1,
                        depth,
                        v.len()
                    );
                }
                for x in v {
                    let inner_values =
                        json_to_input_value(&[*field_type.clone()], &[x.clone()], depth + 1)?;
                    if let Some(inner_value) = inner_values.first() {
                        array_values.push(inner_value.clone());
                    }
                }

                let values = BoundedVec(array_values);
                args.push(InputValue::Array(*field_type.clone(), values));
            }
            FieldType::List(field_type) => {
                let mut list_values = Vec::new();

                let v = value.as_array().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected list value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;

                for x in v {
                    let inner_values =
                        json_to_input_value(&[*field_type.clone()], &[x.clone()], depth + 1)?;
                    if let Some(inner_value) = inner_values.first() {
                        list_values.push(inner_value.clone());
                    }
                }

                let values = BoundedVec(list_values);
                args.push(InputValue::List(*field_type.clone(), values));
            }
            FieldType::Struct(field_types) => {
                let mut struct_values = Vec::new();
                let v = value.as_object().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected struct value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;

                let mut obj = v.iter();
                for field_type in &field_types.0 {
                    if let Some((key, val)) = obj.next() {
                        let inner_values =
                            json_to_input_value(&[field_type.clone()], &[val.clone()], depth + 1)?;
                        if let Some(inner_value) = inner_values.first() {
                            struct_values.push((new_bounded_string(key), inner_value.clone()));
                        }
                    }
                }

                let values = BoundedVec(struct_values);
                args.push(InputValue::Struct(
                    new_bounded_string(format!("struct_{}", i + depth)),
                    Box::new(values),
                ));
            }
            FieldType::AccountId => {
                let v = value.as_str().ok_or_else(|| {
                    color_eyre::eyre::eyre!(
                        "Expected AccountId value for parameter {} (depth: {depth})",
                        i + 1
                    )
                })?;
                // Parse the account ID from the string
                match AccountId32::from_str(v) {
                    Ok(account_id) => args.push(InputValue::AccountId(account_id)),
                    Err(_) => {
                        return Err(color_eyre::eyre::eyre!(
                            "Invalid AccountId format (depth: {depth})"
                        ));
                    }
                }
            }
        }
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use tangle_subxt::FieldExt;

    use super::*;

    #[test]
    fn it_converts_json_to_field() {
        let input = serde_json::json!({
            "a": 1,
            "b": true,
            "c": "hello",
            "d": [1, 2, 3],
            "e": {
                "f": 4,
                "g": [5, 6]
            },
            "h": null,
            "i": [1, 3, 5, 7],
            "j": {
                "k": {
                    "l": "world"
                }
            }
        });
        let expected = vec![InputValue::Struct(
            new_bounded_string("struct_0"),
            Box::new(BoundedVec(vec![
                (new_bounded_string("a"), InputValue::Uint32(1)),
                (new_bounded_string("b"), InputValue::Bool(true)),
                (
                    new_bounded_string("c"),
                    InputValue::String(new_bounded_string("hello")),
                ),
                (
                    new_bounded_string("d"),
                    InputValue::Array(
                        FieldType::Uint32,
                        BoundedVec(vec![
                            InputValue::Uint32(1),
                            InputValue::Uint32(2),
                            InputValue::Uint32(3),
                        ]),
                    ),
                ),
                (
                    new_bounded_string("e"),
                    InputValue::Struct(
                        new_bounded_string("struct_1"),
                        Box::new(BoundedVec(vec![
                            (new_bounded_string("f"), InputValue::Uint32(4)),
                            (
                                new_bounded_string("g"),
                                InputValue::Array(
                                    FieldType::Uint32,
                                    BoundedVec(vec![InputValue::Uint32(5), InputValue::Uint32(6)]),
                                ),
                            ),
                        ])),
                    ),
                ),
                (
                    new_bounded_string("h"),
                    InputValue::Optional(FieldType::String, Box::new(None)),
                ),
                (
                    new_bounded_string("i"),
                    InputValue::List(
                        FieldType::Uint32,
                        BoundedVec(vec![
                            InputValue::Uint32(1),
                            InputValue::Uint32(3),
                            InputValue::Uint32(5),
                            InputValue::Uint32(7),
                        ]),
                    ),
                ),
                (
                    new_bounded_string("j"),
                    InputValue::Struct(
                        new_bounded_string("struct_1"),
                        Box::new(BoundedVec(vec![(
                            new_bounded_string("k"),
                            InputValue::Struct(
                                new_bounded_string("struct_2"),
                                Box::new(BoundedVec(vec![(
                                    new_bounded_string("l"),
                                    InputValue::String(new_bounded_string("world")),
                                )])),
                            ),
                        )])),
                    ),
                ),
            ])),
        )];

        let param_types = vec![FieldType::Struct(Box::new(BoundedVec(vec![
            FieldType::Uint32,
            FieldType::Bool,
            FieldType::String,
            FieldType::Array(3, Box::new(FieldType::Uint32)),
            FieldType::Struct(Box::new(BoundedVec(vec![
                FieldType::Uint32,
                FieldType::Array(2, Box::new(FieldType::Uint32)),
            ]))),
            FieldType::Optional(Box::new(FieldType::String)),
            FieldType::List(Box::new(FieldType::Uint32)),
            FieldType::Struct(Box::new(BoundedVec(vec![FieldType::Struct(Box::new(
                BoundedVec(vec![FieldType::String]),
            ))]))),
        ])))];
        let result = json_to_input_value(&param_types, &[input], 0).unwrap();
        assert_eq!(
            result,
            expected,
            "Expected: {}, but got: {}",
            serde_json::to_string_pretty(
                &from_field::<serde_json::Value>(expected[0].clone()).unwrap()
            )
            .unwrap(),
            serde_json::to_string_pretty(
                &from_field::<serde_json::Value>(result[0].clone()).unwrap()
            )
            .unwrap()
        );
        // sainty check
        for (i, (param_type, value)) in param_types.iter().zip(result.iter()).enumerate() {
            assert_eq!(
                param_type,
                &value.field_type(),
                "Parameter type mismatch at index {}: expected {:?}, got {:?}",
                i,
                param_type,
                value.field_type()
            );
        }
    }
}
