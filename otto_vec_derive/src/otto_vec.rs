fn extend_ident_with_vec(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{}_vec", ident), ident.span())
}

pub fn impl_otto_vec_macro(ast: syn::ItemFn) -> proc_macro::TokenStream {
    if parse_arguments_number(&ast) == 0 {
        panic!("Function must have at least 1 argument.")
    } else {
        let function_name = parse_ident(&ast);
        let function_generics = parse_generics(&ast);
        let function_arguments = parse_arguments(&ast);
        let size = function_arguments.len();
        let (function_arguments_name, function_arguments_name_vec, function_arguments_type) =
            unzip_triple(function_arguments.into_iter());
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
                let sizes = [#(#function_arguments_name_vec.len()),*];
                if sizes.iter().all(|x| x.eq(sizes.first().unwrap())) {
                    let size = sizes.first().unwrap();
                    let mut result = std::vec::Vec::with_capacity(#size);
                    for _ in 0..*size {
                        #(
                            let #function_arguments_name = #function_arguments_name_vec.pop().unwrap();
                        )*
                        result.push((||{
                            #function_body
                        })())
                    }
                    result
                } else {
                    panic!("The size of vectors are not equal.");
                }
            }
        })
    }
}

fn parse_ident(ast: &syn::ItemFn) -> syn::Ident {
    extend_ident_with_vec(&ast.sig.ident)
}

fn parse_generics(ast: &syn::ItemFn) -> syn::ImplGenerics<'_> {
    let (impl_generics, _, _) = ast.sig.generics.split_for_impl();
    impl_generics
}

fn parse_arguments_number(ast: &syn::ItemFn) -> usize {
    ast.sig.inputs.iter().len()
}

fn parse_arguments(ast: &syn::ItemFn) -> Vec<(&syn::Pat, syn::Ident, &syn::Type)> {
    ast.sig
        .inputs
        .iter()
        .enumerate()
        .map(|(index, input)| match input {
            syn::FnArg::Receiver(_) => {
                panic!("Vectorization on member functions is not currently supported.")
            }
            syn::FnArg::Typed(pat_type) => (
                &*pat_type.pat,
                match &*pat_type.pat {
                    syn::Pat::Ident(syn::PatIdent { ident, .. }) => extend_ident_with_vec(&ident),
                    _ => syn::Ident::new(&format!("arg_{}", index), proc_macro2::Span::call_site()),
                },
                &*pat_type.ty,
            ),
        })
        .collect()
}

fn parse_return(ast: &syn::ItemFn) -> syn::Type {
    if let syn::ReturnType::Type(_, b) = &ast.sig.output {
        *b.clone()
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

fn unzip_triple<A, B, C>(input: impl Iterator<Item = (A, B, C)>) -> (Vec<A>, Vec<B>, Vec<C>) {
    let hint = input.size_hint();
    let size = if let Some(size) = hint.1 {
        size
    } else {
        hint.0
    };
    let mut a = Vec::<A>::with_capacity(size);
    let mut b = Vec::<B>::with_capacity(size);
    let mut c = Vec::<C>::with_capacity(size);
    for x in input.into_iter() {
        let (x, y, z) = x;
        a.push(x);
        b.push(y);
        c.push(z);
    }
    (a, b, c)
}
