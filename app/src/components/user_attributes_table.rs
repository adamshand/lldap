use crate::{
    components::delete_user_attribute::DeleteUserAttribute,
    infra::common_component::{CommonComponent, CommonComponentParts},
};
use anyhow::{Error, Result};
use graphql_client::GraphQLQuery;
use yew::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../schema.graphql",
    query_path = "queries/get_user_attributes_schema.graphql",
    response_derives = "Debug,Clone,PartialEq,Eq",
    custom_scalars_module = "crate::infra::graphql"
)]
pub struct GetUserAttributesSchema;

use get_user_attributes_schema::ResponseData;

pub type Attribute = get_user_attributes_schema::GetUserAttributesSchemaSchemaUserSchemaAttributes;
pub type AttributeType = get_user_attributes_schema::AttributeType;

pub struct UserAttributesTable {
    common: CommonComponentParts<Self>,
    attributes: Option<Vec<Attribute>>,
}

pub enum Msg {
    ListAttributesResponse(Result<ResponseData>),
    OnAttributeDeleted(String),
    OnError(Error),
}

impl CommonComponent<UserAttributesTable> for UserAttributesTable {
    fn handle_msg(&mut self, _: &Context<Self>, msg: <Self as Component>::Message) -> Result<bool> {
        match msg {
            Msg::ListAttributesResponse(schema) => {
                self.attributes = Some(schema?.schema.user_schema.attributes.into_iter().collect());
                Ok(true)
            }
            Msg::OnError(e) => Err(e),
            Msg::OnAttributeDeleted(attribute_name) => {
                debug_assert!(self.attributes.is_some());
                self.attributes
                    .as_mut()
                    .unwrap()
                    .retain(|a| a.name != attribute_name);
                Ok(true)
            }
        }
    }

    fn mut_common(&mut self) -> &mut CommonComponentParts<Self> {
        &mut self.common
    }
}

impl Component for UserAttributesTable {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut table = UserAttributesTable {
            common: CommonComponentParts::<Self>::create(),
            attributes: None,
        };
        table.common.call_graphql::<GetUserAttributesSchema, _>(
            ctx,
            get_user_attributes_schema::Variables {},
            Msg::ListAttributesResponse,
            "Error trying to fetch user schema",
        );
        table
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        CommonComponentParts::<Self>::update(self, ctx, msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
              {self.view_attributes(ctx)}
              {self.view_errors()}
            </div>
        }
    }
}

impl UserAttributesTable {
    fn view_attributes(&self, ctx: &Context<Self>) -> Html {
        let make_table = |attributes: &Vec<Attribute>| {
            html! {
                <div class="table-responsive">
                  <table class="table table-hover">
                    <thead>
                      <tr>
                        <th>{"Attribute name"}</th>
                        <th>{"Type"}</th>
                        <th>{"Editable"}</th>
                        <th>{"Visible"}</th>
                        <th>{"Delete"}</th>
                      </tr>
                    </thead>
                    <tbody>
                      {attributes.iter().map(|u| self.view_attribute(ctx, u)).collect::<Vec<_>>()}
                    </tbody>
                  </table>
                </div>
            }
        };
        match &self.attributes {
            None => html! {{"Loading..."}},
            Some(attributes) => make_table(attributes),
        }
    }

    fn view_attribute(&self, ctx: &Context<Self>, attribute: &Attribute) -> Html {
        let link = ctx.link();
        let attribute_type = match attribute.attribute_type {
            AttributeType::STRING => "String",
            AttributeType::INTEGER => "Integer",
            AttributeType::JPEG_PHOTO => "Jpeg",
            AttributeType::DATE_TIME => "DateTime",
            _ => "Unknown",
        };
        let checkmark = html! {
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-check" viewBox="0 0 16 16">
          <path d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425z"></path>
        </svg>
                };
        html! {
          <tr key={attribute.name.clone()}>
              <td>{&attribute.name}</td>
              <td>{if attribute.is_list { format!("List<{attribute_type}>")} else {attribute_type.to_string()}}</td>
              <td>{if attribute.is_editable {checkmark.clone()} else {html!{}}}</td>
              <td>{if attribute.is_visible {checkmark.clone()} else {html!{}}}</td>
              <td>{if attribute.is_hardcoded {html!{
                <button
                    class="btn btn-danger"
                    disabled=true>
                    <i class="bi-x-circle-fill" aria-label="Delete attribute" />
                </button>
              }} else {html!{
                <DeleteUserAttribute
                    attribute_name={attribute.name.clone()}
                  on_attribute_deleted={link.callback(Msg::OnAttributeDeleted)}
                  on_error={link.callback(Msg::OnError)}/>
            }}}</td>
          </tr>
        }
    }

    fn view_errors(&self) -> Html {
        match &self.common.error {
            None => html! {},
            Some(e) => html! {<div>{"Error: "}{e.to_string()}</div>},
        }
    }
}
