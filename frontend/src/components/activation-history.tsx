import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { Alert, AlertDescription } from '@/components/ui/alert'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Skeleton } from '@/components/ui/skeleton'

interface ActivationHistoryProps {
  userId: string
}

export function ActivationHistory({ userId }: ActivationHistoryProps) {
  const {
    data: activations,
    isPending,
    isError,
  } = useQuery({
    queryKey: ['activations', userId],
    queryFn: async () => {
      const { data, error } = await api.GET('/api/users/{user_id}/activations', {
        params: { path: { user_id: userId } },
      })
      if (error) throw error
      return data
    },
  })

  if (isPending) return <Skeleton className="h-40" />

  if (isError) {
    return (
      <Alert variant="destructive">
        <AlertDescription>Не удалось загрузить историю активаций</AlertDescription>
      </Alert>
    )
  }

  if (activations?.length === 0) {
    return <p className="text-muted-foreground text-sm">Активаций пока не было</p>
  }

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Продукт</TableHead>
          <TableHead>Когда</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {activations?.map((activation) => (
          <TableRow key={activation.id}>
            <TableCell>{activation.product_name}</TableCell>
            <TableCell>{new Date(activation.activated_at).toLocaleString('ru-RU')}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  )
}
