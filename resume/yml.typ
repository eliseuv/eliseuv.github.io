#import "template.typ": edu_item, exp_item, header, project_item, publication_item, resume, resume_heading, skill_item

#let get_env_or(key, fallback) = {
  let val = sys.inputs.at(key, default: fallback)
  if val == "" { fallback } else { val }
}

#let yml_personal(d) = {
  header(
    name: get_env_or("RESUME_NAME", d.at("name", default: "")),
    phone: get_env_or("RESUME_PHONE", d.at("phone", default: "")),
    email: get_env_or("RESUME_EMAIL", d.at("email", default: "")),
    website: d.website,
    linkedin: d.linkedin,
    github: d.github,
    orcid: d.orcid,
  )
}

#let yml_education(d) = {
  resume_heading[Education]
  for edu in d {
    edu_item(
      name: edu.name,
      degree: edu.degree,
      location: edu.location,
      date: edu.date,
      logo: edu.at("logo", default: none),
      degree_logo: edu.at("degree_logo", default: none),
      url: edu.at("link", default: none),
      degree_url: edu.at("degree_link", default: none),
      ..edu.points.map(s => eval(s, mode: "markup")),
    )
  }
}

#let yml_experience(d) = {
  resume_heading[Experience]
  for exp in d {
    exp_item(
      role: exp.role,
      name: exp.name,
      location: exp.location,
      date: exp.date,
      logo: exp.at("logo", default: none),
      url: exp.at("link", default: none),
      ..exp.points.map(s => eval(s, mode: "markup")),
    )
  }
}

#let yml_projects(d) = {
  resume_heading[Projects]
  for proj in d {
    project_item(
      name: proj.name,
      skills: proj.skills.join(" | "),
      repo: proj.at("repo", default: none),
      ..proj.points.map(s => eval(s, mode: "markup")),
    )
  }
}

#let yml_publications(d) = {
  resume_heading[Publications]
  for pub in d {
    publication_item(
      doi: pub.doi,
      year: pub.year,
      title: pub.title,
      journal: pub.journal,
      authors: pub.authors.join(", "),
    )
  }
}

#let yml_skills(d) = {
  resume_heading[Skills]
  for skill in d {
    let pairs = skill.pairs().first()
    skill_item(
      category: pairs.at(0),
      skills: pairs.at(1).join(", "),
    )
  }
}

#let yml_resume(data) = {
  show: resume
  yml_personal(data.personal)
  yml_skills(data.skills)
  yml_experience(data.experience)
  yml_education(data.education)
  yml_projects(data.projects)
  yml_publications(data.publications)
}
