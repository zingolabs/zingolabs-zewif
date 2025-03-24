#[macro_export]
macro_rules! impl_attachable {
    ($type:ty) => {
        impl $crate::Attachable for $type {
            fn attachments(&self) -> &Attachments {
                &self.attachments
            }

            fn attachments_mut(&mut self) -> &mut Attachments {
                &mut self.attachments
            }
        }
    };
}
