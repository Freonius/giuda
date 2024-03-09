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
            let mut out_class: File =
                File::create(&format!("{}{}altacv.cls", my_cwd, MAIN_SEPARATOR))
                    .expect("Unable to create file");
            out_class
                .write_all(get_class().as_bytes())
                .expect("Unable to write data");
        } else {
            println!("{}", doc);
        }
    }
}

fn get_class() -> String {
    return r#"
%%%%%%%%%%%%%%%%%
% This is altacv.cls (v1.3.2, 17 May 2020) written by
% LianTze Lim (liantze@gmail.com).
%
%% It may be distributed and/or modified under the
%% conditions of the LaTeX Project Public License, either version 1.3
%% of this license or (at your option) any later version.
%% The latest version of this license is in
%%    http://www.latex-project.org/lppl.txt
%% and version 1.3 or later is part of all distributions of LaTeX
%% version 2003/12/01 or later.
%%
%%
% Contributions:
% - https://github.com/akreuzer Added ragged2e option (5 Nov 2018)
% - https://github.com/stefanogermano Fixed bad boxes and undefined font shape (July 2018)
% - https://github.com/foohyfooh Fixed blank spaces in \cvevent and bad link in README.md (June 2018)

%%%%%%%%%%%%%%%%
\NeedsTeXFormat{LaTeX2e}[1995/12/01]
\ProvidesClass{altacv}[2020/05/17 AltaCV v1.3.2, yet another alternative class for a resume/curriculum vitae.]

%% v1.1: Optionally load academicons
\newif\if@academicons
\DeclareOption{academicons}{\@academiconstrue}
%% v1.1.3: Choice of round/square photo
\newif\if@normalphoto
\DeclareOption{normalphoto}{\@normalphototrue}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{extarticle}}
\newif\if@raggedtwoe
\DeclareOption{ragged2e}{\@raggedtwoetrue}
%% v1.3: load hyperref for clickable hyperlinks
\newif\if@withhyper
\DeclareOption{withhyper}{\@withhypertrue}
\ProcessOptions\relax

\LoadClass{article}
%% v1.3.2 Hopefully this helps make the PDF
%% file more 'friendly' with copy-paste etc
\RequirePackage[a-1b]{pdfx}
\RequirePackage[margin=2cm]{geometry}
\RequirePackage[fixed]{fontawesome5}
\RequirePackage{ifxetex,ifluatex}
\RequirePackage{scrlfile}
\RequirePackage{xparse}

%% v1.1.5: added for convenience
\newif\ifxetexorluatex
\ifxetex
    \xetexorluatextrue
\else
    \ifluatex
    \xetexorluatextrue
    \else
    \xetexorluatexfalse
    \fi
\fi

\ifxetexorluatex
    \RequirePackage{fontspec}
\else
    %% v1.3.2 attempts to make ligatures
    %% copy-paste as normal characters
    \RequirePackage{cmap}
    \RequirePackage[utf8]{inputenc}
    \RequirePackage[T1]{fontenc}
    \input{glyphtounicode}
    \pdfglyphtounicode{f_f}{FB00}
    \pdfglyphtounicode{f_f_i}{FB03}
    \pdfglyphtounicode{f_f_l}{FB04}
    \pdfglyphtounicode{f_i}{FB01}
    \pdfgentounicode=1
\fi

%% v1.1: Optionally load academicons
%% v1.1.5: Handle different versions of academicons
\if@academicons
    \ifxetexorluatex
    \RequirePackage{fontspec}
    %% academicons in TL2018 doesn't require
    %% Academicons to be installed in OS fonts
    %% so can be loaded directly
    \@ifl@t@r\fmtversion{2018/04/01}{%
        \RequirePackage{academicons}
    }{%
        % TL2017
        \@ifl@t@r\fmtversion{2017/04/01}{%
        \@ifpackagelater{academicons}{2018/03/01}{%
            \RequirePackage{academicons}
        }{%
            \let\ori@newfontfamily\newfontfamily%
            \renewcommand{\newfontfamily}[2]{}
            \RequirePackage{academicons}
            \let\newfontfamily\ori@newfontfamily
            \newfontfamily{\AI}{academicons.ttf}
        }
        }{% TL2016 requires the package to be loaded before
        % the version can be checked. Only added because
        % Overleaf v1 still runs TL2016; will be removed
        % when v1 is completely retired.
            \let\ori@newfontfamily\newfontfamily%
            \renewcommand{\newfontfamily}[2]{}
            \RequirePackage{academicons}
            \let\newfontfamily\ori@newfontfamily
            \newfontfamily{\AI}{academicons.ttf}
        }
    }
    \else
    \ClassError{AltaCV}{academicons unsupported by latex or pdflatex. Please compile with xelatex or lualatex}{Please compile with xelatex or lualatex to use the academicons option}
    \fi
\fi

\if@raggedtwoe
    \RequirePackage[newcommands]{ragged2e}
\fi

\if@withhyper
    \AtBeginDocument{%
    \RequirePackage{hyperref}
    \hypersetup{hidelinks}
    \urlstyle{same}
    }
\fi

\RequirePackage{xcolor}

\colorlet{accent}{blue!70!black}
\colorlet{emphasis}{black}
\colorlet{heading}{black}
\colorlet{headingrule}{black}
\colorlet{subheading}{emphasis}
\colorlet{body}{black!80!white}
\colorlet{name}{heading}
\colorlet{tagline}{accent}
\newcommand{\itemmarker}{{\small\textbullet}}
\newcommand{\ratingmarker}{\faCircle}

\RequirePackage{tikz}
\usetikzlibrary{arrows}
\RequirePackage[skins]{tcolorbox}
\RequirePackage[inline]{enumitem}
\setlist{leftmargin=*,labelsep=0.5em,nosep,itemsep=0.25\baselineskip,after=\vspace{0.25\baselineskip}}
\setlist[itemize]{label=\itemmarker}
\RequirePackage{graphicx}
\RequirePackage{etoolbox}
\RequirePackage{dashrule}
\RequirePackage{multirow,tabularx}
\RequirePackage{changepage}
% \RequirePackage{marginfix}

\setlength{\parindent}{0pt}
\newcommand{\divider}{\textcolor{body!30}{\hdashrule{\linewidth}{0.6pt}{0.5ex}}\medskip}

\newenvironment{fullwidth}{%
    \begin{adjustwidth}{}{\dimexpr-\marginparwidth-\marginparsep\relax}}
    {\end{adjustwidth}}

%% v1.3.1 \detokenize will break UTF-8 in pdflatex
%% Using alternative from https://tex.stackexchange.com/a/530911/226
\newcommand{\utffriendlydetokenize}[1]{%
\scantokens{%
    \catcode`\_=12%
%   \catcode`\^=12%
%   \catcode`\{=12%
%   \catcode`\}=12%
    \catcode`\&=12%
    \catcode`\$=12%
    \catcode`\#=12%
    \catcode`\~=12%
%   \catcode`\\=12%
    {#1}%
}%
}
%% v1.3: Incorporating hyperlinks
%% v1.3.1: using \unfriendlydetokenize to avoid
%% breaking unicode
\ExplSyntaxOn
\NewDocumentCommand{\printinfo}{m m o}{%
    \IfNoValueTF{#3}{%
    \mbox{\textcolor{accent}{\normalfont #1}~\utffriendlydetokenize{#2}\hspace{2em}}%
    }{%
        \if@withhyper%
        \mbox{\textcolor{accent}{\normalfont #1}~
        \href{#3}{\utffriendlydetokenize{#2}}\hspace{2em}}
        \else%
        \ClassWarning{Please specify [withhyper] option to enable hyperlinks. Printing out full hyperlink prefix #1 for now.}%
        \mbox{\textcolor{accent}{\normalfont #1}~{\utffriendlydetokenize{#3#2}}\hspace{2em}}%
        \fi%
    }%
}%

%% v1.3: Exploring convenient creation of fields
\NewDocumentCommand{\NewInfoField}{m m o}{%
    \IfNoValueF{#3}{\csdef{#1 hyperprefix}{#3}}%
    \csdef{#1 symbol}{#2}%
    \csdef{#1}##1{%
    \if@withhyper
        \IfNoValueTF {#3}
        {\printinfo{\csuse{#1 symbol}}{##1}}%
        {\printinfo{\csuse{#1 symbol}}{##1}[\csuse{#1 hyperprefix}##1]}%
    \else
        \printinfo{\csuse{#1 symbol}}{##1}%
    \fi%
    }
}
\ExplSyntaxOff

\newcommand{\name}[1]{\def\@name{#1}}
\newcommand{\tagline}[1]{\def\@tagline{#1}}
\newcommand{\personalinfo}[1]{\def\@personalinfo{#1}}
\NewInfoField{email}{\faAt}[mailto:]
\NewInfoField{mailaddress}{\faEnvelope}
\NewInfoField{phone}{\faPhone}
\NewInfoField{homepage}{\faGlobe}[https://]
\NewInfoField{twitter}{\faTwitter}[https://twitter.com/]
\NewInfoField{linkedin}{\faLinkedin}[https://linkedin.com/in/]
\NewInfoField{github}{\faGithub}[https://github.com/]
\NewInfoField{behance}{\faBehance}[https://www.behance.net/]
\NewInfoField{dockerhub}{\faDocker}[https://hub.docker.com/u/]
\NewInfoField{instagram}{\faInstagram}[https://www.instagram.com/]
\NewInfoField{npm}{\faNpm}[https://www.npmjs.com/~]
\NewInfoField{medium}{\faMedium}[https://medium.com/@]
\NewInfoField{spotify}{\faSpotify}[https://open.spotify.com/user/]
\NewInfoField{soundcloud}{\faSoundcloud}[https://soundcloud.com/]
\NewInfoField{youtube}{\faYoutube}[https://youtube.com/@]
\NewInfoField{blog}{\faBlog}[https://]
\NewInfoField{orcid}{\aiOrcid}[https://orcid.org/]
\NewInfoField{location}{\faMapMarker}

% v1.2: Support for multiple photos
\newlength{\altacv@photos@width}
\newlength{\altacv@photo@diam@left}
\newlength{\altacv@photo@diam@right}
\def\altacv@left@photos{}
\def\altacv@right@photos{}

\newcommand{\@makeaphoto}[2]{%
    \begin{minipage}{#1}%
    \if@normalphoto
        \includegraphics[width=\linewidth]{#2}
    \else
        \tikz\path[fill overzoom image={#2}]circle[radius=0.5\linewidth];
    \fi%
    \end{minipage}%
}

\newcommand{\altacv@add@photo@left}[1]{%
    \appto{\altacv@left@photos}{%
    \@makeaphoto{\altacv@photo@diam@left}{#1}\hspace{1ex}%
    }%
    \addtolength{\altacv@photos@width}{\altacv@photo@diam@left}%
    \addtolength{\altacv@photos@width}{1ex}%
}
\newcommand{\altacv@add@photo@right}[1]{%
    \appto{\altacv@right@photos}{%
    \@makeaphoto{\altacv@photo@diam@right}{#1}\hspace{1ex}%
    }%
    \addtolength{\altacv@photos@width}{\altacv@photo@diam@right}%
    \addtolength{\altacv@photos@width}{1ex}%
}
\newcommand{\photoL}[2]{%
    \setlength{\altacv@photo@diam@left}{#1}%
    \forcsvlist{\altacv@add@photo@left}{#2}%
}
\newcommand{\photoR}[2]{%
    \setlength{\altacv@photo@diam@right}{#1}%
    \forcsvlist{\altacv@add@photo@right}{#2}%
}
\let\photo\photoR

\newcommand{\namefont}{\Huge\bfseries}
\newcommand{\taglinefont}{\large\bfseries}
\newcommand{\personalinfofont}{\footnotesize\bfseries}
\newcommand{\cvsectionfont}{\LARGE\bfseries}
\newcommand{\cvsubsectionfont}{\large\bfseries}

\newcommand{\makecvheader}{%
    \begingroup
    \altacv@left@photos\hfill%
    \begin{minipage}{\dimexpr\linewidth-\altacv@photos@width}%
    \raggedright%
    {\namefont\color{name}\MakeUppercase{\@name}\par}
    \medskip
    {\taglinefont\color{tagline}\@tagline\par}
    \medskip
    {\personalinfofont\@personalinfo\par}
    \end{minipage}\hfill%
    \altacv@right@photos\par%
    \endgroup\medskip
}

\renewenvironment{quote}{\color{accent}\itshape\large}{\par}

\newcommand{\cvsection}[2][]{%
    \bigskip%
    \ifstrequal{#1}{}{}{\marginpar{\vspace*{\dimexpr1pt-\baselineskip}\raggedright\input{#1}}}%
    {\color{heading}\cvsectionfont\MakeUppercase{#2}}\\[-1ex]%
    {\color{headingrule}\rule{\linewidth}{2pt}\par}\medskip
}

\newcommand{\cvsubsection}[1]{%
    \smallskip%
    {\color{subheading}\cvsubsectionfont{#1}\par}\medskip
}

% v1.1.4: fixes inconsistent font size
\newcommand{\cvevent}[4]{%
    {\large\color{emphasis}#1\par}
    \smallskip\normalsize
    \ifstrequal{#2}{}{}{
    \textbf{\color{accent}#2}\par
    \smallskip}
    \ifstrequal{#3}{}{}{{\small\makebox[0.5\linewidth][l]{\faCalendar~#3}}}%
    \ifstrequal{#4}{}{}{{\small\makebox[0.5\linewidth][l]{\faMapMarker~#4}}}\par
    \medskip\normalsize
}

\newcommand{\cvachievement}[3]{%
    \begin{tabularx}{\linewidth}{@{}p{2em} @{\hspace{1ex}} >{\raggedright\arraybackslash}X@{}}
    \multirow{2}{*}{\Large\color{accent}#1} & \bfseries\textcolor{emphasis}{#2}\\
    & #3
    \end{tabularx}%
    \smallskip
}

\newcommand{\cvtag}[1]{%
    \tikz[baseline]\node[anchor=base,draw=body!30,rounded corners,inner xsep=1ex,inner ysep =0.75ex,text height=1.5ex,text depth=.25ex]{#1};
}

\newcommand{\cvskill}[2]{%
\textcolor{emphasis}{\textbf{#1}}\hfill
\foreach \x in {1,...,5}{%
    \space{\ifnumgreater{\x}{#2}{\color{body!30}}{\color{accent}}\ratingmarker}}\par%
}

% Adapted from @Jake's answer at http://tex.stackexchange.com/a/82729/226
\newcommand{\wheelchart}[4][0]{%
    \begingroup\centering
    \def\innerradius{#3}%
    \def\outerradius{#2}%
    % Calculate total
    \pgfmathsetmacro{\totalnum}{0}%
    \foreach \value/\colour/\name in {#4} {%
        \pgfmathparse{\value+\totalnum}%
        \global\let\totalnum=\pgfmathresult%
    }%
    \begin{tikzpicture}

        % Calculate the thickness and the middle line of the wheel
        \pgfmathsetmacro{\wheelwidth}{\outerradius-\innerradius}
        \pgfmathsetmacro{\midradius}{(\outerradius+\innerradius)/2}
        \pgfmathsetmacro{\totalrot}{-90 + #1}

        % Rotate so we start from the top
        \begin{scope}[rotate=\totalrot]

        % Loop through each value set. \cumnum keeps track of where we are in the wheel
        \pgfmathsetmacro{\cumnum}{0}
        \foreach \value/\width/\colour/\name in {#4} {
            \pgfmathsetmacro{\newcumnum}{\cumnum + \value/\totalnum*360}

            % Calculate the percent value
            \pgfmathsetmacro{\percentage}{\value/\totalnum*100}
            % Calculate the mid angle of the colour segments to place the labels
            \pgfmathsetmacro{\midangle}{-(\cumnum+\newcumnum)/2}

            % This is necessary for the labels to align nicely
            \pgfmathparse{
                (-\midangle>180?"west":"east")
            } \edef\textanchor{\pgfmathresult}
            \pgfmathparse{
                (-\midangle>180?"flush left":"flush right")
            } \edef\textalign{\pgfmathresult}
            \pgfmathsetmacro\labelshiftdir{1-2*(-\midangle<180)}

            % Draw the color segments. Somehow, the \midrow units got lost, so we add 'pt' at the end. Not nice...
            \filldraw[draw=white,fill=\colour] (-\cumnum:\outerradius) arc (-\cumnum:-(\newcumnum):\outerradius) --
            (-\newcumnum:\innerradius) arc (-\newcumnum:-(\cumnum):\innerradius) -- cycle;

            % Draw the data labels
            \draw  [*-,thin,emphasis] node [append after command={(\midangle:\midradius pt) -- (\midangle:\outerradius + 1ex) -- (\tikzlastnode)}] at (\midangle:\outerradius + 1ex) [xshift=\labelshiftdir*0.5cm,inner sep=1ex, outer sep=0pt, text width=\width,anchor=\textanchor,align=\textalign,font=\small,text=body]{\name};
            % Set the old cumulated angle to the new value
            \global\let\cumnum=\newcumnum
        }
        \end{scope}
%      \draw[gray] (0,0) circle (\outerradius) circle (\innerradius);
    \end{tikzpicture}\par
    \endgroup
}

\newcommand{\cvref}[3]{%
    \smallskip
    \textcolor{emphasis}{\textbf{#1}}\par
    \begin{description}[font=\color{accent},style=multiline,leftmargin=1.35em,align=left]
    \item[\small\normalfont\emailsymbol] #2
    \item[\small\normalfont\mailaddresssymbol] #3
    \end{description}
%   \medskip
}

\newenvironment{cvcolumn}[1]{\begin{minipage}[t]{#1}\raggedright}{\end{minipage}}

%\RequirePackage[backend=biber,style=authoryear,sorting=ydnt]{biblatex}
%% For removing numbering entirely when using a numeric style
% \setlength{\bibhang}{1em}
% \DeclareFieldFormat{labelnumberwidth}{\makebox[\bibhang][l]{\itemmarker}}
% \setlength{\biblabelsep}{0pt}
%\defbibheading{pubtype}{\cvsubsection{#1}}
%\renewcommand{\bibsetup}{\vspace*{-\baselineskip}}
%\AtEveryBibitem{\makebox[\bibhang][l]{\itemmarker}}
%\setlength{\bibitemsep}{0.25\baselineskip}

% v1.1.2: make it easier to add a sidebar aligned with top of next page
\RequirePackage{afterpage}
\newcommand{\addsidebar}[2][]{\marginpar{%
    \ifstrequal{#1}{}{}{\vspace*{#1}}%
    \input{#2}}%
}
\newcommand{\addnextpagesidebar}[2][]{\afterpage{\addsidebar[#1]{#2}}}

\AtBeginDocument{%
    \pagestyle{empty}
    \color{body}
    \raggedright
}
    
"#.to_string();
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
        if doc["break_after"]
            .as_bool()
            .is_some_and(|break_after: bool| break_after == true)
        {
            experiences.push_str("\\newpage\n\n");
        } else {
            experiences.push_str("\n\n\\divider\n\n");
        }
    });
    experiences = experiences.trim_end_matches("\\divider\n\n").to_string();
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

fn get_educations_or_certifications(yaml: &Yaml, key: &str) -> String {
    let mut out: String = String::new();
    yaml[key].as_vec().unwrap().iter().for_each(|doc: &Yaml| {
        let doc: &Yaml = doc;
        let institution: String = doc["institution"].as_str().unwrap().to_string();
        let date: String = doc["date"].as_str().unwrap().to_string();
        let title: String = doc["title"].as_str().unwrap().to_string();
        let location: String = doc["location"].as_str().unwrap().to_string();

        out.push_str(&format!(
            "\\cvevent{{{}}}{{{}}}{{{}}}{{{}}}\n",
            escape_latex(&title),
            escape_latex(&institution),
            escape_latex(&date),
            escape_latex(&location)
        ));
        if doc["description"].as_str().is_some() {
            out.push_str(&format!(
                "{}\n",
                escape_latex(&doc["description"].as_str().unwrap().to_string())
            ));
        }
    });
    return out;
}

fn get_publications(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    let mut in_list: bool = false;
    yaml["publications"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            if doc.as_str().is_some() {
                if in_list == false {
                    out.push_str("\\begin{itemize}\n");
                    in_list = true;
                }
                out.push_str(&format!(
                    "\\item {}\n",
                    escape_latex(&doc.as_str().unwrap().to_string())
                ));
            } else {
                if in_list == true {
                    out.push_str("\\end{itemize}\n");
                }
                in_list = false;
                let title: String = doc["title"].as_str().unwrap().to_string();
                out.push_str(&format!("\\textbf{{{}}}\n\n\\begin{{itemize}}\n", title));
                doc["publications"]
                    .as_vec()
                    .unwrap()
                    .to_vec()
                    .iter()
                    .for_each(|item: &Yaml| {
                        let item: &Yaml = item;
                        out.push_str(&format!(
                            "\\item {}\n",
                            escape_latex(&item.as_str().unwrap().to_string())
                        ));
                    });
                out.push_str("\\end{itemize}\n\n");
            }
        });
    if in_list == true {
        out.push_str("\\end{itemize}\n");
    }
    return out;
}

fn get_strengths(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    yaml["strengths"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            out.push_str(&format!(
                "\\cvtag{{{}}}\n",
                escape_latex(&doc.as_str().unwrap().to_string())
            ));
        });
    return out;
}

fn get_hobbies(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    yaml["hobbies"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            if doc.as_str().is_some() {
                out.push_str(&format!(
                    "\\cvtag{{{}}}\n",
                    escape_latex(&doc.as_str().unwrap().to_string())
                ));
            } else {
                let title: String = doc["name"].as_str().unwrap().to_string();
                let icon: String = doc["icon"].as_str().unwrap().to_string();
                out.push_str(&format!(
                    "\\cvtag{{\\fa{} {}}}\n",
                    icon,
                    escape_latex(&title)
                ));
            }
        });
    return out;
}

fn get_projects(yaml: &Yaml) -> String {
    let mut out: String = String::new();
    yaml["projects"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            let title: String = escape_latex(&doc["title"].as_str().unwrap().to_string());
            let description: String =
                escape_latex(&doc["description"].as_str().unwrap().to_string());
            let name: String = escape_latex(&doc["name"].as_str().unwrap().to_string());
            let mut date: String = String::new();
            if doc["date"].as_str().is_some() {
                date = escape_latex(&doc["date"].as_str().unwrap().to_string());
            }
            let mut url: String = String::new();
            if doc["link"].as_str().is_some() {
                let addr: String = escape_latex(&doc["link"].as_str().unwrap().to_string());
                url = format!("\\href{{{}}}{{{}}}\n\n", addr, addr);
            }
            out.push_str(&format!(
                "\\cvevent{{{}}}{{{}}}{{{}}}{{}}\n{}{}\n\n\\divider\n",
                name, title, date, url, description
            ));
        });
    out = out.trim_end_matches("\\divider\n").to_string();
    return out;
}

fn escape_latex(text: &String) -> String {
    return text
        .replace("\\", "\\\\")
        .replace("&", "\\&")
        .replace("#", "\\#")
        .replace("%", "\\%")
        .replace("$", "\\$")
        .replace("[i]", "\\textit{")
        .replace("[/i]", "}")
        .replace("[b]", "\\textbf{")
        .replace("[/b]", "}");
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
                "{}\n\\medskip\n",
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
        if section_type == "education" {
            out.push_str(&format!(
                "{}",
                get_educations_or_certifications(full, "education")
            ))
        }
        if section_type == "certifications" {
            out.push_str(&format!(
                "{}",
                get_educations_or_certifications(full, "certifications")
            ))
        }
        if section_type == "publications" {
            out.push_str(&format!("{}", get_publications(full)));
        }
        if section_type == "strengths" {
            out.push_str(&format!("{}", get_strengths(full)));
        }
        if section_type == "hobbies" {
            out.push_str(&format!("{}", get_hobbies(full)));
        }
        if section_type == "projects" {
            out.push_str(&format!("{}", get_projects(full)));
        }
        if doc["break_after"]
            .as_bool()
            .is_some_and(|break_after: bool| break_after == true)
        {
            out.push_str("\\newpage\n\n");
        }
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
    } else {
        out.push_str("\\tagline{}\n");
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
    yaml["links"]
        .as_vec()
        .unwrap()
        .iter()
        .for_each(|doc: &Yaml| {
            let doc: &Yaml = doc;
            out.push_str(&format!(
                "\\{}{{{}}}\n",
                escape_latex(&doc["type"].as_str().unwrap().to_string()),
                escape_latex(&doc["link"].as_str().unwrap().to_string())
            ))
        });
    out.push_str(&format!(
        "  \\location{{{}}}\n",
        escape_latex(&yaml["location"].as_str().unwrap().to_string())
    ));
    out.push_str("}%\n\n");
    let side_is_left: bool = yaml["side_is_left"]
        .as_bool()
        .is_some_and(|is_left: bool| is_left);
    if side_is_left {
        out.push_str("\\makecvheader\n\n\\columnratio{0.4}\n\n\\begin{paracol}{2}\n");
    } else {
        out.push_str("\\makecvheader\n\n\\columnratio{0.6}\n\n\\begin{paracol}{2}\n");
    }
    let main_section: &Vec<Yaml> = yaml["main_section"].as_vec().unwrap();
    let side_section: &Vec<Yaml> = yaml["side_section"].as_vec().unwrap();
    if side_is_left {
        out.push_str(&format!("{}", get_section(&side_section, &yaml)));
    } else {
        out.push_str(&format!("{}", get_section(&main_section, &yaml)));
    }
    out.push_str(&format!("\\switchcolumn\n\n"));
    if side_is_left {
        out.push_str(&format!("{}", get_section(&main_section, &yaml)));
    } else {
        out.push_str(&format!("{}", get_section(&side_section, &yaml)));
    }
    out.push_str("\n\\end{paracol}\n\\end{document}");

    return out;
}
