import { schema, denormalize } from 'normalizr'
import { Project } from '../../types'

export type NormalizedProject = Omit<Project, 'files'> & {
  files: string[]
}

export const fileEntity = new schema.Entity(
  'files',
  {},
  {
    idAttribute: 'path',
  }
)

export const projectEntity = new schema.Entity(
  'projects',
  {
    files: [fileEntity],
  },
  {
    idAttribute: 'path',
  }
)

export const denormalizeProject = (
  normalizedProject: NormalizedProject
): Project => {
  return denormalize(normalizedProject, projectEntity, fileEntity)
}
