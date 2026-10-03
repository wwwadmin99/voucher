import createClient from 'openapi-fetch'
import type { paths } from './api-types'

export const api = createClient<paths>({ baseUrl: '/api' })

export type { components } from './api-types'
