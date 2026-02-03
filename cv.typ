#let cv = yaml("cv.yaml")

#set document(title: cv.personal.name, author: cv.personal.name)
#set page(margin: (x: 2cm, y: 2cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)

// Colors
#let accent = rgb("#f74c00")
#let dark = rgb("#2a3439")

// Header
#align(center)[
  #text(size: 24pt, weight: "bold", fill: dark)[#cv.personal.name]
  #v(4pt)
  #text(size: 12pt, fill: accent)[#cv.personal.title]
  #v(4pt)
  #text(size: 10pt)[#cv.personal.location]
  #v(8pt)
  #text(size: 9pt)[
    #link("mailto:" + cv.personal.email)[#cv.personal.email] |
    #link(cv.personal.github)[GitHub] |
    #link(cv.personal.linkedin)[LinkedIn]
  ]
]

#v(16pt)

// Summary
#text(size: 12pt, weight: "bold", fill: accent)[Summary]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)
#cv.personal.intro

#v(12pt)

// Experience
#text(size: 12pt, weight: "bold", fill: accent)[Experience]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)

#for job in cv.experience [
  #grid(
    columns: (1fr, auto),
    text(weight: "bold")[#job.title #text(fill: dark.lighten(30%))[at] #job.company],
    text(style: "italic", size: 9pt)[#job.date]
  )
  #v(2pt)
  #job.description
  #v(2pt)
  #text(size: 8pt, fill: dark.lighten(40%))[#job.skills.join(" · ")]
  #v(8pt)
]

// Skills
#text(size: 12pt, weight: "bold", fill: accent)[Skills]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)

#grid(
  columns: (auto, 1fr),
  row-gutter: 4pt,
  column-gutter: 12pt,
  text(weight: "bold")[Languages:], cv.skills.languages.join(", "),
  text(weight: "bold")[Frameworks:], cv.skills.frameworks.join(", "),
  text(weight: "bold")[Infrastructure:], cv.skills.infrastructure.join(", "),
  text(weight: "bold")[Observability:], cv.skills.observability.join(", "),
  text(weight: "bold")[Data:], cv.skills.data.join(", "),
)

#v(12pt)

// Education
#text(size: 12pt, weight: "bold", fill: accent)[Education]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)

#for edu in cv.education [
  #grid(
    columns: (1fr, auto),
    [#text(weight: "bold")[#edu.degree] \ #edu.institution],
    text(style: "italic", size: 9pt)[#edu.date]
  )
  #v(4pt)
]

#v(12pt)

// Interests
#text(size: 12pt, weight: "bold", fill: accent)[Interests]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)

#for interest in cv.interests [
  #text(weight: "bold")[#interest.name] --- #interest.description
  #v(4pt)
]

#v(12pt)

// Work Preferences
#text(size: 12pt, weight: "bold", fill: accent)[Work Preferences]
#line(length: 100%, stroke: 0.5pt + accent)
#v(4pt)

#grid(
  columns: (auto, 1fr),
  row-gutter: 4pt,
  column-gutter: 12pt,
  text(weight: "bold")[Employment:], cv.work_preferences.employment,
  text(weight: "bold")[Arrangements:], cv.work_preferences.arrangements.join(", "),
  text(weight: "bold")[Travel:], cv.work_preferences.travel,
)
