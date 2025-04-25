use crate::{Args, Block, BracketDirection, Item, Template};

/// The amount of blocks in a code block.
const CODE_BLOCK_SIZE: usize = 2;

/// Splits a list of templates into smaller templates if they are too large.
///
/// Warning: There are a few assumptions made in this function:
/// - The function depth of individual blocks do not matter.
/// - Global scope is not overly used (i.e. if statements should be a few blocks maximum).
/// - Non-function templates cannot be split.
pub fn split_templates(templates: Vec<Template>, max_size: usize) -> Vec<Template> {
    let mut stack = templates;
    let mut result = Vec::new();

    while let Some(mut template) = stack.pop() {
        // If the template is small enough, add it to the result
        if template.blocks.len() * CODE_BLOCK_SIZE <= max_size {
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
                    get_next_function_name(name)
                } else {
                    panic!("Functions with parameters are not supported for splitting");
                }
            }
            _ => panic!("Non-functions are not supported for splitting"),
        };

        // Calculate the last global scope
        let last_global_scope = find_last_global_scope(&template.blocks, max_size);

        // Create a new
        let mut right_half =
            Vec::with_capacity((template.blocks.len() - last_global_scope + 5) * 2);

        right_half.push(Block::Function {
            args: Args::default(),
            name: new_function_name.clone(),
        });

        // Split the blocks at the last global scope
        split_into_existing(&mut template.blocks, last_global_scope, &mut right_half);

        // Insert the new function call at the end of the left half
        template.blocks.push(Block::CallFunction {
            args: Args::default(),
            func: new_function_name,
        });

        // The left half is now a valid template less than max_size, so we can add it to the result.
        result.push(template);

        // Now verify that the right half is also not too large
        stack.push(Template::new(right_half));
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
                if (i + 1) * CODE_BLOCK_SIZE > max_size {
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
fn get_next_function_name(s: &str) -> String {
    if let Some(pos) = s.rfind("--") {
        let suffix = &s[pos + 2..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(n) = suffix.parse::<u32>() {
                return format!("{}--{}", &s[..pos], n + 1);
            }
        }
    }

    format!("{s}--1",)
}

/// Splits a vector into two parts at the given index, moving the second part into the target vector.
fn split_into_existing<T>(vec: &mut Vec<T>, at: usize, target: &mut Vec<T>) {
    let drained = vec.drain(at..);
    target.extend(drained);
}
