use std::collections::HashSet;

use crate::{Args, Block, BracketDirection, Item, Template};

/// The amount of blocks in a code block.
const CODE_BLOCK_SIZE: usize = 2;
const SAFE_SPACE: usize = 4;

/// Splits a list of templates into smaller templates if they are too large.
///
/// Warning: There are a few assumptions made in this function:
/// - The function depth of individual blocks do not matter.
/// - Global scope is not overly used (i.e. if statements should be a few blocks maximum).
/// - Non-function templates cannot be split.
pub fn split_templates(templates: Vec<Template>, max_size: usize) -> Vec<Template> {
    let mut function_names = HashSet::new();

    for template in &templates {
        if let Some(Block::Function { name, .. }) = template.blocks.first() {
            function_names.insert(name.clone());
        }
    }

    let mut stack = templates;
    let mut result = Vec::new();

    while let Some(mut template) = stack.pop() {
        // If the template is small enough, add it to the result
        if template.blocks.len() * CODE_BLOCK_SIZE + SAFE_SPACE <= max_size {
            result.push(template);
            continue;
        }

        // Get the new name of the template
        let new_function_name = match template.blocks.first() {
            Some(Block::Function { args, name }) => {
                let mut non_tag_args = args
                    .0
                    .iter()
                    .filter(|(_, item)| !matches!(item, Item::Tag { .. }));

                if non_tag_args.next().is_none() {
                    get_next_function_name(name, &mut function_names)
                } else {
                    panic!("Functions with parameters are not supported for splitting");
                }
            }
            _ => panic!("Non-functions are not supported for splitting"),
        };

        // Calculate the last global scope
        let last_global_scope = find_last_global_scope(&template.blocks, max_size);

        // Create a new
        let mut right_template = Template::start_function_hidden(new_function_name.clone());

        // Split the blocks at the last global scope
        split_into_existing(
            &mut template.blocks,
            last_global_scope,
            &mut right_template.blocks,
        );

        // Insert the new function call at the end of the left half
        template.blocks.push(Block::CallFunction {
            args: Args::default(),
            func: new_function_name,
        });

        // The left half is now a valid template less than max_size, so we can add it to the result.
        result.push(template);

        // Now verify that the right half is also not too large
        stack.push(right_template);
    }

    result
}

/// Finds the last global scope less than `max_size` in the given blocks.
fn find_last_global_scope(blocks: &[Block], max_size: usize) -> usize {
    let mut bracket_depth = 0;
    let mut last_global_scope = 0;

    for (i, block) in blocks.iter().enumerate() {
        match block {
            Block::IfEntity { .. }
            | Block::IfGame { .. }
            | Block::IfPlayer { .. }
            | Block::IfVariable { .. }
            | Block::Else => {
                // Skip these blocks, cannot split here
            }
            Block::Bracket { direction, .. } => match direction {
                BracketDirection::Open => bracket_depth += 1,
                BracketDirection::Close => bracket_depth -= 1,
            },
            _ => {
                if (i + 1) * CODE_BLOCK_SIZE + SAFE_SPACE > max_size {
                    // We have reached the max size, so stop searching
                    break;
                } else if bracket_depth == 0 {
                    // We are at the global scope, so we can split here
                    last_global_scope = i;
                }
            }
        }
    }

    last_global_scope
}

/// Given a function name, returns a new unique function name by incrementing the number at the end of the name.
fn get_next_function_name(s: &str, function_names: &mut HashSet<String>) -> String {
    let (prefix, mut n) = if let Some(pos) = s.rfind("--") {
        let prefix = &s[..pos];
        let suffix = &s[pos + 2..];

        if let Ok(n) = suffix.parse::<usize>() {
            (prefix.to_string(), n)
        } else {
            (s.to_string(), 0)
        }
    } else {
        (s.to_string(), 0)
    };

    loop {
        n += 1;
        let new_name = format!("{}--{}", prefix, n);
        if !function_names.contains(&new_name) {
            function_names.insert(new_name.clone());
            return new_name;
        }
    }
}

/// Splits a vector into two parts at the given index, moving the second part into the target vector.
fn split_into_existing<T>(vec: &mut Vec<T>, at: usize, target: &mut Vec<T>) {
    let drained = vec.drain(at..);
    target.extend(drained);
}
