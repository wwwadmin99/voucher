import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { useState } from 'react'
import { api } from '@/lib/api'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface AdminPanelProps {
  userId: string
}

export function AdminPanel({ userId }: AdminPanelProps) {
  const queryClient = useQueryClient()
  const [productId, setProductId] = useState<string>('')
  const [quantity, setQuantity] = useState<string>('')
  const [feedback, setFeedback] = useState<{ type: 'success' | 'error'; message: string } | null>(
    null,
  )

  const { data: products } = useQuery({
    queryKey: ['products'],
    queryFn: async () => {
      const { data, error } = await api.GET('/api/products')
      if (error) throw error
      return data
    },
  })

  const setBalance = useMutation({
    mutationFn: async () => {
      const { data, error } = await api.PUT('/api/users/{user_id}/balances/{product_id}', {
        params: { path: { user_id: userId, product_id: productId } },
        body: { quantity: Number(quantity) },
      })
      if (error) throw error
      return data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['balances', userId] })
      setFeedback({ type: 'success', message: 'Запас обновлён' })
    },
    onError: (error) => {
      const message =
        error instanceof Object && 'message' in error
          ? String((error as { message: unknown }).message)
          : 'Не удалось обновить запас'
      setFeedback({ type: 'error', message })
    },
  })

  return (
    <Card>
      <CardHeader>
        <CardTitle>Задать запас ваучеров</CardTitle>
      </CardHeader>
      <CardContent>
        <form
          className="flex flex-col gap-4"
          onSubmit={(e) => {
            e.preventDefault()
            setFeedback(null)
            setBalance.mutate()
          }}
        >
          <div className="flex flex-col gap-2">
            <Label htmlFor="admin-product">Продукт</Label>
            <Select value={productId} onValueChange={setProductId}>
              <SelectTrigger id="admin-product">
                <SelectValue placeholder="Выберите продукт" />
              </SelectTrigger>
              <SelectContent>
                {products?.map((product) => (
                  <SelectItem key={product.id} value={product.id}>
                    {product.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="flex flex-col gap-2">
            <Label htmlFor="admin-quantity">Количество</Label>
            <Input
              id="admin-quantity"
              type="number"
              min={0}
              value={quantity}
              onChange={(e) => setQuantity(e.target.value)}
              required
            />
          </div>

          {feedback && (
            <Alert variant={feedback.type === 'error' ? 'destructive' : 'default'}>
              <AlertDescription>{feedback.message}</AlertDescription>
            </Alert>
          )}

          <Button type="submit" disabled={!productId || quantity === '' || setBalance.isPending}>
            Сохранить
          </Button>
        </form>
      </CardContent>
    </Card>
  )
}
