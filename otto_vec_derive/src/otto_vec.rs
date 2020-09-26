pub fn impl_otto_vec_macro(ast: syn::ItemFn) -> proc_macro::TokenStream {
    let function_name = parse_ident(&ast);
    let function_generics = parse_generics(&ast);
    let (function_arguments_name, function_arguments_name_vec, function_arguments_type) =
        parse_arguments(&ast);
    let size = function_arguments_name.len();
    let function_return_type = parse_return(&ast);
    let function_body = parse_body(&ast);
    proc_macro::TokenStream::from(quote::quote! {
        #ast
        pub fn
        #function_name
        #function_generics
        (#(
            mut #function_arguments_name_vec : std::vec::Vec<#function_arguments_type>
        ),*)
        ->
        std::vec::Vec<#function_return_type> {
            #(
                #function_arguments_name_vec.reverse();
            )*
            let mut sizes = std::vec::Vec::with_capacity(#size);
            #(sizes.push(#function_arguments_name_vec.len()));*;
            if sizes.iter().all(|x| x.eq(sizes.first().unwrap())) {
                let size = sizes.first().unwrap();
                let mut result = std::vec::Vec::with_capacity(#size);
                for i in 0..*size {
                    #(
                        let mut #function_arguments_name = #function_arguments_name_vec.pop().unwrap();
                    )*
                    result.push((||{
                        #function_body
                    })())
                }
                result
            } else {
                panic!("size of vectors are not equal.");
            }
        }
    })
}

fn parse_ident(ast: &syn::ItemFn) -> syn::Ident {
    syn::Ident::new(&format!("{}_vec", &ast.sig.ident), ast.sig.ident.span())
}

fn parse_generics(ast: &syn::ItemFn) -> syn::ImplGenerics<'_> {
    let (impl_generics, _, _) = ast.sig.generics.split_for_impl();
    impl_generics
}

fn parse_arguments(
    ast: &syn::ItemFn,
) -> (
    Vec<&Box<syn::Pat>>,
    Vec<proc_macro2::Ident>,
    Vec<&Box<syn::Type>>,
) {
    (
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => &pat_type.pat,
            })
            .collect(),
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                    syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
                        syn::Ident::new(&format!("{}_vec", &ident), ident.span())
                    }
                    syn::Pat::Tuple(pat_tuple) => {
                        panic!("Unsupported tuple pattern\n{:#?}", pat_tuple)
                    }
                    v => panic!("unsupported pattern type\n{:#?}", v),
                },
            })
            .collect(),
        ast.sig
            .inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Vectorization on member functions is not currently supported.")
                }
                syn::FnArg::Typed(pat_type) => &pat_type.ty,
            })
            .collect(),
    )
}

fn parse_return(ast: &syn::ItemFn) -> syn::Type {
    if let syn::ReturnType::Type(_, b) = &ast.sig.output {
        return *b.clone();
    } else {
        panic!("Vectorized function must have a return type.")
    }
}

fn parse_body(ast: &syn::ItemFn) -> proc_macro2::TokenStream {
    let statements = ast.block.stmts.iter().map(|statement| statement);
    quote::quote! {
        #(#statements)*
    }
}
