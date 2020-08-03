use crate::xsd::{attribute::Attribute, XsdContext};
use log::info;
use proc_macro2::TokenStream;
use std::io::prelude::*;
use yaserde::YaDeserialize;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "annotation"
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
  )]
pub struct Annotation {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(
      rename = "documentation"
      prefix = "xs",
      namespace = "xs: http://www.w3.org/2001/XMLSchema"
    )]
  pub documentation: Vec<String>,
}

impl Annotation {
  pub fn get_implementation(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    info!("Generate annotation");

    let documentation = self.documentation.iter().map(|documentation| {
      quote! {
        #[doc = #documentation]
      }
    });

    quote! {
      #(#documentation)*
    }
  }
}
