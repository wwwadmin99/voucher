import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useState } from 'react'
import { api } from '@/lib/api'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'

interface VoucherBoardProps {
  userId: string
}

export function VoucherBoard({ userId }: VoucherBoardProps) {
  const queryClient = useQueryClient()
  const [errors, setErrors] = useState<Record<string, string>>({})

  const {
    data: balances,
    isPending,
    isError,
  } = useQuery({
    queryKey: ['balances', userId],
    queryFn: async () => {
      const { data, error } = await api.GET('/api/users/{user_id}/balances', {
        params: { path: { user_id: userId } },
      })
      if (error) throw error
      return data
    },
  })

  const activate = useMutation({
    mutationFn: async (productId: string) => {
      const { data, error } = await api.POST('/api/users/{user_id}/activations', {
        params: { path: { user_id: userId } },
        body: { product_id: productId },
      })
      if (error) throw error
      return data
    },
    onMutate: (productId) => {
      setErrors((prev) => ({ ...prev, [productId]: '' }))
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['balances', userId] })
      queryClient.invalidateQueries({ queryKey: ['activations', userId] })
    },
    onError: (error, productId) => {
      const message =
        error instanceof Object && 'message' in error
          ? String((error as { message: unknown }).message)
          : 'Не удалось активировать ваучер'
      setErrors((prev) => ({ ...prev, [productId]: message }))
    },
  })

  if (isPending) {
    return (
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {[0, 1, 2].map((i) => (
          <Skeleton key={i} className="h-36" />
        ))}
      </div>
    )
  }

  if (isError) {
    return (
      <Alert variant="destructive">
        <AlertDescription>Не удалось загрузить баланс ваучеров</AlertDescription>
      </Alert>
    )
  }

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {balances?.map((balance) => {
        const outOfVouchers = balance.quantity <= 0
        const error = errors[balance.product_id]
        return (
          <Card key={balance.product_id}>
            <CardHeader>
              <CardTitle>{balance.product_name}</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-3xl font-semibold">{balance.quantity}</p>
              <p className="text-muted-foreground text-sm">доступно ваучеров</p>
              {error && <p className="text-destructive mt-2 text-sm">{error}</p>}
              {!error && outOfVouchers && (
                <p className="text-muted-foreground mt-2 text-sm">Ваучеров не осталось</p>
              )}
            </CardContent>
            <CardFooter>
              <Button
                className="w-full"
                disabled={outOfVouchers || activate.isPending}
                onClick={() => activate.mutate(balance.product_id)}
              >
                Активировать
              </Button>
            </CardFooter>
          </Card>
        )
      })}
    </div>
  )
}
