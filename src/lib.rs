use std::time::Duration;
use middle_wasm::prelude::*;

#[middle_fn]
/// This function returns `Hello, {input.name}`
fn make_hello(name: String) -> String {
    mprint("you can print to the console by calling mprint()");
    format!("Hello, {name}!")
}

#[middle_workflow]
/// This is a test workflow, which can pause, and stop and resume later.
fn test_workflow(to_print: String) -> Resumable<String> {
    // This function prints something to Middle's console.
    mprint("making a request...");

    // This code will repeatedly execute until a non-500 error is returned.
    // There's an upper-bound to the number of steps executed, after which an error will result.
    let out = loop {
        let out = RequestBuilder::get("https://middle.app")
            .call()
            .unwrap();

        if out.code() >= 500 {
            // Waiting 5 seconds for 5xx error to resolve
            middle_wasm::pause(Duration::from_secs(5))?;
        } else {
            break out;
        }
    };
    
    // Code from this point on will only be run once the pause duration has passed.
    mprint(format!("we were supposed to print... {to_print}"));
    mprint(format!("here's the result of our call... {}", out.code()));
    
    // We have to return Ready when the workflow is done.
    Resumable::Ready("I'm done!".to_string())
}

#[middle_workflow]
/// This test workflow pauses once and then returns.
fn test_pause() -> Resumable<()> {
    mprint(format!("pausing for 5 seconds..."));
    middle_wasm::pause(Duration::from_secs(5))?;
    mprint(format!("pause complete!"));
    Resumable::Ready(())
}

#[derive(JsonSchema, Deserialize, Serialize)]
struct Contact {
    /// First name
    #[validate(length(min = 1, max = 100))]
    first: String,

    /// Last name
    #[validate(length(min = 1, max = 100))]
    last: String,
}

#[derive(JsonSchema, Deserialize)]
/// Choose a contact, or indicate that you're done.
enum ContactPrompt {
    /// Do you want to enter another contact?
    Contact(Contact),

    /// All done?
    Done
}

#[middle_workflow]
/// This example workflow prompts for user input.
/// If a user selects to run this workflow, the workflow will stop at each "prompt" and ask for input before continuing.
/// In this way, you can create a "wizard" flow that, step by step, accepts and processes a series of user inputs.
fn collect_contacts() -> Resumable<Result<Vec<Contact>, String>> {
    let mut contacts = vec![];

    loop {
        // Call `prompt`, a generic function whose single type must be transformable into a JSON-schema.
        // We will use that schema to generate a nice form for the user to work with.
        // Prompt may return an error, which we unwrap. 
        // This schema can be an enum, which is helpful for loops like this.
        let contact_or_done = match prompt::<ContactPrompt>()? {
            Ok(contact_or_done) => contact_or_done,
            Err(error) => {
                mprint(format!("error from prompt: {error:?}"));
                return Resumable::Ready(Err("Error from prompt!".to_string()));
            },
        };
        match contact_or_done {
            ContactPrompt::Contact(contact) => {
                mprint(format!("Hello, {} {}", contact.first, contact.last));
                contacts.push(contact);
            },
            ContactPrompt::Done => break
        }
    }

    // We do nothing with the contacts we collected, other than print the length to the console.
    // But imagine, you could post the contacts to a CRM, or an email, or some other useful thing!
    mprint(format!("All done! {} contacts found", contacts.len()));

    Resumable::Ready(Ok(contacts))
}