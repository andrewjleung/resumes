#let template(resume) = [
  #set line(length: 100%, stroke: 0.5pt + black)
  #set list(indent: 0.5em)
  #set page("us-letter", margin: 1in)
  #set par(linebreaks: "simple", leading: 1em, spacing: 1.4em)
  #set text(font: "Carlito", size: 10pt)

  #show heading.where(level: 1): set text(size: 18pt)

  #let parse(date) = {
    datetime(year: date.year, month: date.month, day: date.day)
  }

  #let headline(name, desc, location: none, detail) = {
    box[#pad(right: 0.25em)[*#name*]]
    [#desc]
    if location != none [
      — #location
    ]
    h(1fr)
    [#detail]
  }

  #let monthrange(startdate, enddate: none) = {
    let format = "[month repr:short] [year repr:full]"

    parse(startdate).display(format)
    if enddate == none [
      \- Present
    ] else [
      \- #parse(enddate).display(format)
    ]
  }

  #let nicelink(l, scheme: "https") = {
    let components = l.split(regex("(://)|:"))

    let (scheme, authority) = if components.len() == 2 {
      components
    } else {
      (scheme, l)
    }

    link(scheme + ":" + authority)[#authority]
  }

  #let twocolumn(leftcontent, rightcontent) = {
    grid(
      columns: 3,
      leftcontent, h(1fr), rightcontent,
    )
  }

  // DOCUMENT START

  #twocolumn(
    [
      = #resume.basics.name
      #nicelink(resume.basics.email, scheme: "mailto")
    ],
    align(
      right,
      (
        nicelink(resume.basics.url),
        resume.basics.profiles.map(profile => nicelink(profile.url)),
      )
        .flatten()
        .join[\ ],
    ),
  )

  == Skills

  #line()

  #(
    resume
      .skills
      .map(skill => [*#skill.name*: #skill.keywords.join[, ]])
      .join[\ ]
  )

  == Work Experience

  #line()

  #(
    resume
      .work
      .map(w => [
        #headline(
          w.name,
          w.position,
          location: w.summary,
          monthrange(w.startDate, enddate: w.endDate),
        )

        #list(..w.highlights)
      ])
      .join()
  )

  == Projects

  #line()

  #(
    resume
      .projects
      .map(project => {
        let detail = if project.url == none {
          "Closed Source"
        } else {
          nicelink(project.url)
        }

        headline(project.name, detail, project.startDate)

        list(..project.highlights)
      })
      .join()
  )

  == Education

  #line()

  #(
    resume
      .education
      .map(e => headline(
        e.institution,
        [#e.studyType, #e.score GPA],
        location: e.area,
        parse(e.endDate).display("[year repr:full]"),
      ))
      .join()
  )
]

#template(json(sys.inputs.data_path))
