use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} file.yaml [output_file]", args[0]);
    } else {
        let file: &str = &args[1].as_str();
        let doc: String = load_file(file);
        if args.len() > 2 {
            let mut my_cwd: String = ".".to_string();
            if let Ok(cwd) = env::current_dir() {
                my_cwd = cwd.to_str().unwrap().to_string();
            }
            let mut out_file =
                File::create(&format!("{}{}{}.tex", my_cwd, MAIN_SEPARATOR, args[2]))
                    .expect("Unable to create file");
            out_file
                .write_all(doc.as_bytes())
                .expect("Unable to write data");
        } else {
            println!("{}", doc);
        }
    }
}

fn get_experience_items(items: &Vec<Yaml>) -> String {
    let mut out: String = String::new();
    let mut in_list: bool = false;
    items.iter().for_each(|item: &Yaml| {
        let item: &Yaml = item;
        if item.as_str().is_some() {
            if in_list == false {
                out.push_str("\\begin{itemize}\n");
                in_list = true;
            }
            out.push_str(&format!(
                "\\item {}\n",
                escape_latex(&item.as_str().unwrap().to_string())
            ));
        } else {
            if in_list == true {
                out.push_str("\\end{itemize}\n");
            }
            in_list = false;
            let title: String = item["title"].as_str().unwrap().to_string();
            out.push_str(&format!("\\textbf{{{}}}\n\n", title));

            if item
                .as_hash()
                .unwrap()
                .contains_key(&Yaml::String("description".to_string()))
            {
                if !item["description"].is_null() {
                    out.push_str(&format!(
                        "{}\n\n",
                        escape_latex(&item["description"].as_str().unwrap().to_string())
                    ))
                }
            }
            out.push_str(&get_experience_items(item["items"].as_vec().unwrap()));
        }
    });
    if in_list == true {
        out.push_str("\\end{itemize}\n");
    }
    return out;
}

fn get_experiences(yaml: &Yaml) -> String {
    let docs: Vec<Yaml> = yaml["experiences"].as_vec().unwrap().to_vec();
    let mut experiences: String = String::new();
    docs.iter().for_each(|doc: &Yaml| {
        let doc: &Yaml = doc;
        // \cvevent{Ideatore e realizzatore}{*Manuzio}{Novembre 2023 --}{Piacenza, IT}
        let role: String = doc["role"].as_str().unwrap().to_string();
        let company: String = doc["company"].as_str().unwrap().to_string();
        let date: String = doc["date"].as_str().unwrap().to_string();
        let location: String = doc["location"].as_str().unwrap().to_string();
        experiences.push_str(&format!(
            "\\cvevent{{{}}}{{{}}}{{{}}}{{{}}}\n",
            escape_latex(&role),
            escape_latex(&company),
            escape_latex(&date),
            escape_latex(&location),
        ));
        experiences.push_str(&get_experience_items(doc["items"].as_vec().unwrap()));
        experiences.push_str("\n\\divider\n\n");
    });
    return experiences;
}

fn get_about(yaml: &Yaml) -> String {
    if yaml["about"].as_str().is_some() {
        return escape_latex(&yaml["about"].as_str().unwrap().to_string()) + "\n\n";
    }
    let mut out: String = String::new();
    yaml["about"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            out.push_str(&format!(
                "{}\n\n",
                escape_latex(&doc.as_str().unwrap().to_string())
            ));
        });
    out.push_str("\n\n");
    return out;
}

fn get_hero(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    yaml["hero"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            let title: String = doc["title"].as_str().unwrap().to_string();
            let description: String = doc["description"].as_str().unwrap().to_string();
            let icon: String = doc["icon"].as_str().unwrap().to_string();
            out.push_str(&format!(
                "\n\\cvachievement{{\\fa{}}}{{{}}}{{{}}}\n\\divider\n",
                icon,
                escape_latex(&title),
                escape_latex(&description)
            ));
        });
    out = out.trim_end_matches("\n\\divider\n").to_string();
    out.push_str("\n\n");
    return out;
}

fn get_skill_item(item: &Yaml) -> String {
    let title: String = item["title"].as_str().unwrap().to_string();
    let level: String = item["level"].as_i64().unwrap().to_string();
    return format!("\\cvskill{{{}}}{{{}}}\n", escape_latex(&title), level);
}

fn get_skills(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    yaml["skills"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            if doc["title"].as_str().is_some() {
                out.push_str(&format!("{}\\smallskip\n", get_skill_item(doc)));
            } else {
                out = out.trim_end_matches("\\smallskip\n").to_string();
                out = out.trim_end_matches("\\divider\n").to_string();
                out.push_str("\\divider\n");
                doc["skills"]
                    .as_vec()
                    .unwrap()
                    .iter()
                    .for_each(|item: &Yaml| {
                        out.push_str(&format!("{}\\smallskip\n", get_skill_item(item)));
                    });
                out = out.trim_end_matches("\\smallskip\n").to_string();
                out.push_str("\\divider\n");
            }
        });
    out = out.trim_end_matches("\\smallskip\n").to_string();
    out = out.trim_end_matches("\\divider\n").to_string();
    out = out.trim_start_matches("\\divider\n").to_string();
    out.push_str("\\medskip\n");
    return out;
}

fn escape_latex(text: &String) -> String {
    return text
        .replace("\\", "\\\\")
        .replace("&", "\\&")
        .replace("#", "\\#");
}

fn get_section(yaml: &Vec<Yaml>, full: &Yaml) -> String {
    let mut out: String = String::new();
    yaml.iter().for_each(|doc: &Yaml| {
        let doc: &Yaml = doc;
        let title: String = doc["title"].as_str().unwrap().to_string();
        let section_type: String = doc["type"].as_str().unwrap().to_string();
        out.push_str(&format!("\\cvsection{{{}}}\n", escape_latex(&title)));
        if doc["description"].as_str().is_some() {
            out.push_str(&format!(
                "{}\n",
                escape_latex(&doc["description"].as_str().unwrap().to_string())
            ));
        }
        if section_type == "experiences" {
            out.push_str(&format!("{}", get_experiences(full)));
        }
        if section_type == "about" {
            out.push_str(&format!("{}", get_about(full)));
        }
        if section_type == "hero" {
            out.push_str(&format!("{}", get_hero(full)));
        }
        if section_type == "skills" {
            out.push_str(&format!("{}", get_skills(full)));
        }
        // TODO: education, publications, strengths, hobbies, certifications, projects
    });
    return out;
}

fn load_file(file: &str) -> String {
    let mut file: File = File::open(file).expect("Unable to open file");
    let mut contents: String = String::new();
    let mut out: String = r#"
\PassOptionsToPackage{dvipsnames}{xcolor}
\documentclass[10pt,a4paper,ragged2e,withhyper]{altacv}
\geometry{left=1.25cm,right=1.25cm,top=1.3cm,bottom=1.3cm,columnsep=1.2cm}
\usepackage{paracol}
\usepackage{changepage}
\newcommand{\forceindent}{\leavevmode{\parindent=6mm\indent}}

\newcommand{\cvkill}[1]{\textcolor{emphasis}{\textbf{#1}}\hfill\par}

\ifxetexorluatex
    \setmainfont{Roboto Slab}
    \setsansfont{Lato}
    \renewcommand{\familydefault}{\sfdefault}
\else
    \usepackage[rm]{roboto}
    \usepackage[defaultsans]{lato}
    \renewcommand{\familydefault}{\sfdefault}
\fi


\definecolor{SlateGrey}{HTML}{2E2E2E}
\definecolor{LightGrey}{HTML}{666666}
\definecolor{DarkPastelRed}{HTML}{450808}
\definecolor{PastelRed}{HTML}{8F0D0D}
\definecolor{GoldenEarth}{HTML}{E7D192}
\colorlet{name}{black}
\colorlet{tagline}{PastelRed}
\colorlet{heading}{DarkPastelRed}
\colorlet{headingrule}{GoldenEarth}
\colorlet{subheading}{PastelRed}
\colorlet{accent}{PastelRed}
\colorlet{emphasis}{SlateGrey}
\colorlet{body}{LightGrey}

\renewcommand{\namefont}{\Huge\rmfamily\bfseries}
\renewcommand{\personalinfofont}{\footnotesize}
\renewcommand{\cvsectionfont}{\LARGE\rmfamily\bfseries}
\renewcommand{\cvsubsectionfont}{\large\bfseries}


\renewcommand{\itemmarker}{{\small\textbullet}}
\renewcommand{\ratingmarker}{\faCircle}

\usepackage{expl3,xparse}
\usepackage{url}
\usepackage{hyperref}
\NewDocumentCommand{\backend}{m}{#1}

\begin{document}
"#
    .to_string();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs: Vec<Yaml> = YamlLoader::load_from_str(&contents).unwrap();
    let yaml: &Yaml = &docs[0];

    let name: String = yaml["name"].as_str().unwrap().to_string();
    out.push_str(&format!("\\name{{{}}}\n", escape_latex(&name)));
    if yaml["role"].as_str().is_some() {
        out.push_str(&format!(
            "\\tagline{{{}}}\n",
            escape_latex(&yaml["role"].as_str().unwrap().to_string())
        ));
    }
    if yaml["image"].as_str().is_some() {
        out.push_str(&format!(
            "\\photoR{{2.8cm}}{{{}}}\n",
            escape_latex(&yaml["image"].as_str().unwrap().to_string())
        ));
    }
    out.push_str("\\personalinfo{%\n");
    out.push_str(&format!(
        "  \\email{{{}}}\n",
        escape_latex(&yaml["email"].as_str().unwrap().to_string())
    ));
    let phone_prefix: String =
        "+".to_owned() + &yaml["phone"]["prefix"].as_i64().unwrap().to_string();
    let phone_number: String = yaml["phone"]["number"].as_str().unwrap().to_string();
    let phone_prefix_wa: String = phone_prefix.chars().filter(|c| c.is_digit(10)).collect();
    let phone_number_wa: String = phone_number.chars().filter(|c| c.is_digit(10)).collect();
    let phone_wa: String = phone_prefix_wa + &phone_number_wa;
    let phone: String = phone_prefix + " " + &phone_number;
    out.push_str(&format!(
        "  \\phone{{\\href{{https://wa.me/{}}}{{{}}}}}\n",
        phone_wa, phone
    ));
    out.push_str(&format!(
        "  \\location{{{}}}\n",
        escape_latex(&yaml["location"].as_str().unwrap().to_string())
    ));
    // TODO: Links
    out.push_str("}%\n\n");
    out.push_str("\\makecvheader\n\n\\columnratio{0.6}\n\n\\begin{paracol}{2}\n");
    let main_section: &Vec<Yaml> = yaml["main_section"].as_vec().unwrap();
    let side_section: &Vec<Yaml> = yaml["side_section"].as_vec().unwrap();
    out.push_str(&format!("{}", get_section(&main_section, &yaml)));
    out.push_str(&format!("\\switchcolumn\n\n"));
    out.push_str(&format!("{}", get_section(&side_section, &yaml)));
    out.push_str("\n\\end{paracol}\n\\end{document}");

    return out;
}
