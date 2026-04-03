#import "@preview/fontawesome:0.6.0": fa-icon

#let static_dir = "../static"

#let resume(body) = {
  set list(indent: 1em)
  show list: set text(size: 0.92em)
  show link: underline
  show link: set underline(offset: 3pt)
  show link: set text(rgb("#000099"))

  set page(
    paper: "a4",
    margin: (x: 1cm, y: 1cm),
  )

  set text(
    size: 11pt,
    font: "New Computer Modern",
  )

  body
}

#let name_header(name) = {
  set text(size: 2.25em)
  [*#name*]
}

#let header(
  name: "[NAME]",
  phone: "[PHONE_NUMBER]",
  website: "[WEBSITE_URL]",
  email: "[EMAIL_ADDRESS]",
  linkedin: "[LINKEDIN_ID]",
  github: "[GITHUB_ID]",
  orcid: "[ORCID_ID]",
  researchgate: "[RESEARCHGATE_ID]",
) = {
  align(
    center,
    block[
      #name_header(name)
      \ #fa-icon("envelope") #link("mailto:" + email)[#email]
      | #fa-icon("globe") #link(website)[#website]
      | #fa-icon("phone") #link("https://wa.me/" + phone.replace(regex("[^0-9]"), ""))[#phone]
      | #fa-icon("linkedin") #link("https://www.linkedin.com/in/" + linkedin)[#linkedin]
      | #fa-icon("github") #link("https://github.com/" + github)[#github]
      | #fa-icon("orcid") #link("https://orcid.org/" + orcid)[ORCID]
    ],
  )
  v(5pt)
}

#let resume_heading(txt) = {
  show heading: set text(size: 0.92em, weight: "regular")
  block[
    = #smallcaps(txt)
    #v(-4pt)
    #line(length: 100%, stroke: 1pt + black)
  ]
}

#let edu_item(
  name: "Sample University",
  degree: "B.S in Foobar",
  location: "Foo, BA",
  date: "Aug. 1600 - May 1750",
  logo: none,
  degree_logo: none,
  url: none,
  degree_url: none,
  ..points,
) = {
  set block(above: 0.7em, below: 1em)
  pad(
    left: 1em,
    right: 0.5em,
    box[
      #let body = [
        #grid(
          columns: (3fr, 2fr),
          align(left)[
            *#name* \
            _#degree _
          ],
          align(right)[
            #location \
            _#date _
          ],
        )
        #list(..points)
      ]
      #if logo != none or degree_logo != none {
        let logo_node = if logo != none {
          let logo_img = image(static_dir + logo, height: 2em)
          let linked = if url != none {
            link(url)[#box[
              #logo_img
              #place(top + right, dx: 0.3em, dy: -0.3em, text(fill: rgb("#000099"), size: 0.6em)[#sym.arrow.tr])
            ]]
          } else { logo_img }
          (align(center + horizon, linked),)
        } else { () }
        let degree_node = if degree_logo != none {
          let degree_img = image(static_dir + degree_logo, height: 2em)
          let linked = if degree_url != none {
            link(degree_url)[#box[
              #degree_img
              #place(top + right, dx: 0.3em, dy: -0.3em, text(fill: rgb("#000099"), size: 0.6em)[#sym.arrow.tr])
            ]]
          } else { degree_img }
          (align(center + horizon, linked),)
        } else { () }

        let logo_col = if logo != none { (4em,) } else { () }
        let degree_col = if degree_logo != none { (2em,) } else { () }

        grid(
          columns: (auto, 1fr),
          gutter: 0.6em,
          align(top, grid(
            columns: logo_col + degree_col,
            gutter: 0.2em,
            ..(logo_node + degree_node),
          )),
          body,
        )
      } else {
        body
      }
    ],
  )
}

#let exp_item(
  name: "Sample Workplace",
  role: "Worker",
  date: "June 1837 - May 1845",
  location: "Foo, BA",
  logo: none,
  url: none,
  ..points,
) = {
  set block(above: 0.7em, below: 1em)
  pad(
    left: 1em,
    right: 0.5em,
    box[
      #let body = [
        #grid(
          columns: (3fr, 2fr),
          align(left)[
            *#role* \
            _#name _
          ],
          align(right)[
            #location \
            _#date _
          ],
        )
        #list(..points)
      ]
      #if logo != none {
        let logo_img = image(static_dir + logo, width: 2.8em)
        let logo_node = if url != none {
          link(url)[#box[
            #logo_img
            #place(top + right, dx: 0.3em, dy: -0.3em, text(fill: rgb("#000099"), size: 0.6em)[#sym.arrow.tr])
          ]]
        } else { logo_img }
        grid(
          columns: (auto, 1fr),
          gutter: 0.6em,
          align(top, logo_node), body,
        )
      } else {
        body
      }
    ],
  )
}

#let publication_item(
  doi: "doi",
  year: 0,
  title: "Example Title",
  journal: "Example Journal",
  authors: "Example Authors",
  ..points,
) = {
  set block(above: 1em, below: 1.5em)
  pad(
    left: 1em,
    right: 0.3em,
    box[
      #grid(
        columns: (13fr, 1fr),
        align(left)[
          #link("https://doi.org/" + doi)[#fa-icon("file-lines") *#title*] \
          _#journal _ \
          #emph(text(size: 9pt)[#authors])
        ],
        align(right)[
          _#year _
        ],
      )
    ],
  )
}

#let project_item(
  name: "Example Project",
  skills: "Programming Language 1, Database3",
  repo: none,
  ..points,
) = {
  set block(above: 0.7em, below: 1em)
  pad(
    left: 1em,
    right: 0.5em,
    box[
      #if repo != none [
        #link("https://github.com/" + repo)[#fa-icon("github") *#name*] | _#skills _
      ] else [
        *#name* | _#skills _
      ]
      #list(..points)
    ],
  )
}

#let skill_item(
  category: "category",
  skills: "Skill1, Skill2",
) = {
  set block(above: 0.7em)
  set text(size: 0.91em)
  pad(left: 1em, right: 0.5em, block[*#category*: #skills])
}
