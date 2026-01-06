#let template(resume) = [
  #set line(length: 100%, stroke: 0.5pt + black)
  #set list(indent: 0.5em)
  #set page("us-letter", margin: 0.75in)
  #set par(linebreaks: "simple", leading: 0.95em, spacing: 1.2em)
  #set text(font: "Nimbus Sans L", size: 10pt)

  #show heading.where(level: 1): set text(size: 18pt)

  #let headline(name, desc, detail: none, timing) = {
    [*#name*] 
    if desc != none [
        #box[#pad(left: 0.2em)[#desc]]
    ]
    if detail != none [
      — #detail
    ]
    h(1fr)
    [#timing]
  }

  #let monthrange(startdate, enddate: none) = {
    let format = "[month repr:short] [year repr:full]"

    startdate.display(format)
    if enddate == none [
      \- Present
    ] else [
      \- #enddate.display(format)
    ]
  }

  #let nicelink(l) = {
    let authority = l.split(regex("(://)|:")).last()
    link(l)[#authority]
  }

  #let twocolumn(leftcontent, rightcontent) = {
    grid(
      columns: 3,
      leftcontent, h(1fr), rightcontent,
    )
  }

  #let location(l) = {
    if l == none {
      return none
    }

    if "CityState" in l {
      return [#l.CityState.at(0), #l.CityState.at(1)]
    } else if "Address" in l {
      return [#l.Address.city, #l.Address.region]
    } else if (
      l == "Remote"
    ) {
      return l
    } else {
      panic("unrecognized location kind", l)
    }
  }

  #let when(w) = {
    if w == none {
      return none
    }

    if "Range" in w {
      return monthrange(w.Range.start, enddate: w.Range.end)
    } else if "Started" in w {
      return monthrange(w.Started)
    } else if "Year" in w {
      return w.Year
    }
  }

  // DOCUMENT START

  #twocolumn(
    [
      = #resume.profile.first_name #resume.profile.last_name
      #if "contact" in resume {
        nicelink("mailto:" + resume.contact.personal_email)
      }
    ],
    align(
      right,
      resume.socials.map(social => nicelink(social.url)).join[\ ],
    ),
  )

  == Skills

  #line()

  #(
    resume
      .skills
      .pairs()
      .map(entry => [*#entry.at(0)*: #entry.at(1).join[, ]])
      .join[\ ]
  )

  == Work Experience

  #line()

  #let work(w) = {
    headline(w.context, w.name, detail: location(w.location), when(w.when))
    list(..w.highlights)
  }

  #(
    resume.experiences.filter(e => e.kind == "work").map(w => work(w)).join()
  )

  == Projects

  #line()

  #let project(p) = {
    let detail = if p.at("url", default: none) == none {
      "Closed Source"
    } else {
      nicelink(p.url)
    }

    if p.at("summary", default: none) == none {
      headline(p.name, none, detail: detail, when(p.when))
    } else {
      headline(p.name, p.summary, detail: detail, when(p.when))
    }

    list(..p.highlights)
  }

  #(
    resume
      .experiences
      .filter(e => e.kind == "project")
      .map(p => project(p))
      .join()
  )

  == Education

  #line()

  #let education(e) = {
    headline(
      e.institution,
      [#e.kind #e.area, #e.score GPA],
      detail: location(e.location),
      e.when.Range.end.display("[month repr:short] [year repr:full]"),
    )
  }

  #resume.education.map(e => education(e)).join()
]

#template(toml(sys.inputs.data_path))
