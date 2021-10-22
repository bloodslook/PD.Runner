use super::*;

pub struct Gen {
    pub relative: &'static str,
    pub root: &'static str,
    // TODO: add module features here so method_features can remove any that are already module features

    // TODO: add features TokenStream so different impls don't have to pass it around or regenerate?
}

impl Gen {
    pub fn absolute() -> Self {
        Gen {
            relative: "",
            root: "",
        }
    }

    pub fn relative(namespace: &'static str) -> Self {
        Gen {
            relative: namespace,
            root: "",
        }
    }

    pub fn namespace(&self, namespace: &str) -> TokenStream {
        if self.relative.is_empty() {
            let mut tokens = TokenStream::with_capacity();

            for namespace in namespace.split('.') {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        } else {
            if namespace == self.relative {
                return TokenStream::new();
            }

            let mut relative = self.relative.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::with_capacity();

            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }

    pub fn class_features(&self, def: &TypeDef) -> BTreeSet<&'static str> {
        if self.root.is_empty() {
            return BTreeSet::new();
        }

        let mut features = BTreeSet::new();

        if let Some(def) = def.default_interface() {
            features.insert(def.namespace());
        }

        features
    }

    pub fn gen_cfg(&self, features: &BTreeSet<&'static str>) -> TokenStream {
        self.gen_cfg_impl(features, false)
    }

    pub fn gen_cfg_not(&self, features: &BTreeSet<&'static str>) -> TokenStream {
        self.gen_cfg_impl(features, true)
    }

    fn gen_cfg_impl(&self, features: &BTreeSet<&'static str>, not: bool) -> TokenStream {
        if self.root.is_empty() {
            return TokenStream::new();
        }

        if features.is_empty() {
            return TokenStream::new();
        }

        let mut tokens = String::new();
        let mut count = 0;

        for feature in features {
            if self.relative == *feature {
                continue;
            }

            if self.relative.len() > feature.len()
                && self.relative.starts_with(feature)
                && self.relative[feature.len()..].starts_with('.')
            {
                continue;
            }

            let feature = &feature[self.root.len() + 1..];
            tokens.push_str(&format!("feature = \"{}\", ", feature.replace('.', "_")));
            count += 1;
        }

        if count == 0 {
            return TokenStream::new();
        }

        let all = count > 1;
        let mut cfg = "#[cfg(".to_string();

        if not {
            cfg.push_str("not(")
        }

        if all {
            cfg.push_str("all(")
        }

        tokens.truncate(tokens.len() - 2);
        cfg.push_str(&tokens);

        if not {
            cfg.push(')');
        }

        if all {
            cfg.push(')');
        }

        cfg.push_str(")]");
        cfg.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(
            Gen::absolute().namespace("Windows.Foundation").as_str(),
            "Windows::Foundation::"
        );

        assert_eq!(
            Gen::relative("Windows")
                .namespace("Windows.Foundation")
                .as_str(),
            "Foundation::"
        );

        assert_eq!(
            Gen::relative("Windows.Foundation")
                .namespace("Windows.Foundation")
                .as_str(),
            ""
        );

        assert_eq!(
            Gen::relative("Windows.Foundation.Collections")
                .namespace("Windows.Foundation")
                .as_str(),
            "super::"
        );
    }
}
