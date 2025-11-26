#let template(resume) = [
  #set line(length: 100%, stroke: 0.5pt + black)
  #set list(indent: 0.5em)
  #set page("us-letter", margin: 1in)
  #set par(linebreaks: "simple", leading: 1em, spacing: 1.2em)
  #set text(font: "Nimbus Sans L", size: 10pt)

  #show heading.where(level: 1): set text(size: 18pt)

  #let parse(date) = {
    datetime(year: date.year, month: date.month, day: date.day)
  }

  #let headline(name, desc, detail: none, timing) = {
    box[#pad(right: 0.4em)[*#name*]]
    [#desc]
    if detail != none [
      — #detail
    ]
    h(1fr)
    [#timing]
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

  // DOCUMENT START

  #twocolumn(
    [
      = #resume.basics.name
      #nicelink("mailto:" + resume.basics.email)
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
          detail: w.at("location", default: none),
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

        if project.description == none {
          headline(project.name, detail, project.startDate)
        } else {
          headline(
            project.name,
            project.description,
            detail: detail,
            project.startDate,
          )
        }

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
        detail: e.area,
        parse(e.endDate).display("[year repr:full]"),
      ))
      .join()
  )
]

#template(json(sys.inputs.data_path))
