/* THIS FILE IS AUTOGENERATED - DO NOT EDIT */
use dom::globalscope::GlobalScope;
use dom::element::Element;
use dom::document::Document;
use dom::xmldocument::XMLDocument;
use dom::node::Node;
use dom::window::Window;
use dom::bindings::conversions::DerivedFrom;
use dom::bindings::inheritance::Castable;
use typeholder::TypeHolderTrait;

#[derive(Copy, Clone)]
pub union TopTypeId {
    pub alone: (),
    pub eventtarget: EventTargetTypeId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EventTargetTypeId {
    EventTarget,
    Node(NodeTypeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeTypeId {
    DocumentFragment,
    DocumentType,
    Document(DocumentTypeId),
}

impl<TH: TypeHolderTrait> Castable for GlobalScope<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<GlobalScope<TH>> for GlobalScope<TH> {}

impl<TH: TypeHolderTrait> Castable for Element<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<Element<TH>> for Element<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<Node<TH>> for Element<TH> {}

impl<TH: TypeHolderTrait> Castable for Node<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<Node<TH>> for Node<TH> {}

impl<TH: TypeHolderTrait> Castable for Window<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<GlobalScope<TH>> for Window<TH> {}

impl<TH: TypeHolderTrait> Castable for XMLDocument<TH> {}
impl<TH: TypeHolderTrait> DerivedFrom<Document<TH>> for XMLDocument<TH> {}

impl<TH: TypeHolderTrait> Castable for Document<TH> {}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DocumentTypeId {
    Document,
    XMLDocument
}
