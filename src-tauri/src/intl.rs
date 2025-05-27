use std::borrow::Cow;

use fluent_bundle::{concurrent::FluentBundle, FluentResource};
use log::error;
use unic_langid::LanguageIdentifier;

use crate::{CommandError, CommandResult};

pub(crate) struct Intl {
    bundle: FluentBundle<FluentResource>
}

impl Intl {
    pub(crate) fn try_new(langid: LanguageIdentifier) -> CommandResult<Self> {
        let ftl_string = "hello-world = Ciao, cane!".to_owned();
        let res = FluentResource::try_new(ftl_string)
            .or_else(|(res, errors)| {
                error!("Errors occurred while trying to create Fluent resource {:?}", errors);

                Ok::<FluentResource, CommandError>(res)
            })?;

        let mut bundle: FluentBundle<FluentResource> =
            FluentBundle::new_concurrent(vec![langid]);

        bundle.add_resource(res)
            .expect("Failed to load Fluent resource");

        Ok(Self { bundle })
    }

    pub(crate) fn t(&self, id: &str) -> CommandResult<Cow<'_, str>> {
        let msg = self.bundle
            .get_message(id)
            .expect("Message doesn't exist.");

        let pattern = msg
            .value()
            .expect("Message has no value");

        let mut errors = vec![];
        let value = self.bundle
            .format_pattern(&pattern, None, &mut errors);

        Ok(value.to_owned())
    }
}
