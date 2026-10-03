import type { paths } from './api-types'

export const routes = {
  users: '/api/users',
  products: '/api/products',
  userBalances: '/api/users/{user_id}/balances',
  userBalance: '/api/users/{user_id}/balances/{product_id}',
  userActivations: '/api/users/{user_id}/activations',
} satisfies Record<string, keyof paths>
