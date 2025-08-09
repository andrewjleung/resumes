#set line(length: 100%, stroke: 0.5pt + black)
#set list(indent: 0.5em)
#set page("us-letter", margin: 1in)
#set par(linebreaks: "simple", leading: 1em, spacing: 1.4em)
#set text(font: "Carlito", size: 10pt)

#show heading.where(level: 1): set text(size: 18pt)

#let headline(name, desc, location: none, detail) = {
  box[#pad(right: 0.25em)[*#name*]]
  [#desc]
  if location != none [
    — #location
  ]
  h(1fr)
  [#detail]
}

#let nicelink(scheme, authority) = link(
  scheme + ":" + authority,
)[#authority]

#let twocolumn(leftcontent, rightcontent) = {
  grid(
    columns: 3,
    leftcontent, h(1fr), rightcontent,
  )
}

// DOCUMENT START

#twocolumn(
  [
    = Andrew Leung,
    #nicelink("mailto", "andrewleung104@gmail.com")
  ],
  align(right)[
    #nicelink("https", "andrewjleung.me") \
    #nicelink("https", "linkedin.com/in/andrewjleung-") \
    #nicelink("https", "github.com/andrewjleung") \
  ],
)

== Skills

#line()

*Programming Languages*: TypeScript, JavaScript, Python, Ruby, SQL, HTML, CSS \
*Tools & Frameworks*: React, Next.js, Tailwind, Ruby on Rails, RSpec, PostgreSQL, Google BigQuery, Git, Neovim

== Work Experience

#line()

#headline(
  "PayPal",
  "Software Engineer",
  location: "Remote",
  [Jun 2023 - Present],
)

- Maintained Braintree's Ruby on Rails application used to disburse \$1.5B a day across 15,000 active merchants
- Led development, coordinated live testing, and served as a subject matter expert for a feature used to withhold \$1.5M a day from high-risk or delinquent merchants
- Automated the migration of \$57M out of a closing account for merchants onboarding to new banking partners
- Enhanced disbursement reconciliation processes by improving data quality and designing queries in Google BigQuery, reducing unreconciled funds by 88% and increasing reconciliation accuracy to 99.7%
- Increased monthly revenue by \$650,000 by correcting fee calculations to use rounding instead of truncation
- Identified and corrected the misallocation of \$80M caused by a regression from a Ruby on Rails upgrade
- Interviewed intern candidates and onboarded engineers with code walkthroughs and pair-programming

#headline(
  "Poloniex",
  "Software Engineer Co-op",
  location: "Remote",
  [Jan 2021 - Aug 2021],
)

- Developed internal tools for managing users, markets, and customer support tickets using TypeScript and React
- Integrated web application with a content management system using GraphQL and Node.js enabling marketing teams to update web content without engineering support
- Contributed support for rule negation to an open source rules engine powering a Node.js permissions service

#headline(
  "Teikametrics",
  "Software Engineer Co-op",
  location: "Boston, MA",
  [Jan 2020 - Aug 2020],
)

- Wrote backend APIs in Scala and implemented UIs with TypeScript, React, and Redux to support Amazon ad campaign creation and display campaign performance metrics to users

== Projects

#line()

#headline("Compiler for Python-like Language", "Closed Source", 2022)

- Wrote a compiler backend in OCaml and a C runtime for a Python-like expression-oriented language
- Supported dynamically-typed values including numbers, Booleans, tuples, strings, and first-class functions
- Managed program memory with mark-and-sweep garbage collection and register allocation
- Wrote over 600 unit tests and end-to-end tests to ensure correctness of compilation using OUnit

== Education

#line()

#headline(
  "Northeastern University",
  "B.S. Computer Science, 3.9/4.0 GPA",
  location: "Boston, MA",
  2022,
)
