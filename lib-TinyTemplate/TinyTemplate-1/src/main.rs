use tinytemplate::TinyTemplate;

fn main() {
    let data = "{#}";

    let mut tpl = tinytemplate::TinyTemplate::new();

    let _ = tpl.add_template("template", data);
}