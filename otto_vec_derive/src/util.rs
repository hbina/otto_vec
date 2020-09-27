pub fn get_ident_if_field_is_option(field: &syn::Field) -> Option<syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = field.ty
    {
        if let Some(path_segment) = segments.first() {
            if path_segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = path_segment.arguments
                {
                    if let Some(syn::GenericArgument::Type(ty)) = args.first() {
                        return Some(ty.clone());
                    }
                }
            }
        }
    }
    None
}
