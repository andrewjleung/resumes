import { ResumeSchema } from '@kurone-kito/jsonresume-types';

type ResumeBasics = Required<
  Pick<NonNullable<ResumeSchema['basics']>, 'name' | 'email' | 'phone' | 'url'>
> & {
  location: Required<
    Pick<
      NonNullable<NonNullable<ResumeSchema['basics']>['location']>,
      'city' | 'region'
    >
  >;
  profiles: Required<
    Pick<
      NonNullable<
        NonNullable<NonNullable<ResumeSchema['basics']>['profiles']>
      >[number],
      'url'
    >
  >[];
};

type ResumeEducation = Required<
  Pick<
    NonNullable<ResumeSchema['education']>[number],
    'institution' | 'area' | 'studyType' | 'startDate' | 'endDate' | 'score'
  >
>;

type ResumeSkill = Required<
  Pick<NonNullable<ResumeSchema['skills']>[number], 'name' | 'keywords'>
>;

type ResumeExperience = Required<
  Pick<
    NonNullable<ResumeSchema['work']>[number],
    'name' | 'location' | 'position' | 'startDate' | 'endDate' | 'highlights'
  >
>;

type ResumeProject = Required<
  Pick<
    NonNullable<ResumeSchema['projects']>[number],
    'name' | 'keywords' | 'highlights' | 'startDate'
  >
> &
  Pick<NonNullable<ResumeSchema['projects']>[number], 'url' | 'endDate'>;

type BareResume = {
  basics: ResumeBasics;
  education: ResumeEducation[];
  skills: ResumeSkill[];
  work: ResumeExperience[];
  projects: ResumeProject[];
};

type _BareResumeCheck = BareResume extends ResumeSchema ? BareResume : never;

export {
  BareResume,
  ResumeBasics,
  ResumeEducation,
  ResumeSkill,
  ResumeExperience,
  ResumeProject,
};
