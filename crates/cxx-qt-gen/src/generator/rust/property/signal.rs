// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::ForeignItemFn;

use crate::{
    generator::naming::{property::QPropertyName, qobject::QObjectName},
    parser::signals::ParsedSignal,
};

pub fn generate(idents: &QPropertyName, qobject_idents: &QObjectName) -> ParsedSignal {
    // We build our signal in the generation phase as we need to use the naming
    // structs to build the signal name
    let cpp_class_rust = &qobject_idents.cpp_class.rust;
    let notify_rust = &idents.notify.rust;
    let method: ForeignItemFn = syn::parse_quote! {
        #[doc = "Notify for the Q_PROPERTY"]
        fn #notify_rust(self: Pin<&mut #cpp_class_rust>);
    };
    ParsedSignal::from_property_method(
        method,
        idents.notify.clone(),
        qobject_idents.cpp_class.rust.clone(),
    )
}
