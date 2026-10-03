import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Skeleton } from '@/components/ui/skeleton'

interface UserSelectProps {
  userId: string | null
  onChange: (userId: string) => void
}

export function UserSelect({ userId, onChange }: UserSelectProps) {
  const { data: users, isPending, isError } = useQuery({
    queryKey: ['users'],
    queryFn: async () => {
      const { data, error } = await api.GET('/api/users')
      if (error) throw error
      return data
    },
  })

  if (isPending) return <Skeleton className="h-10 w-64" />
  if (isError) return <p className="text-destructive text-sm">Не удалось загрузить пользователей</p>

  return (
    <Select value={userId ?? undefined} onValueChange={onChange}>
      <SelectTrigger className="w-64">
        <SelectValue placeholder="Выберите пользователя" />
      </SelectTrigger>
      <SelectContent>
        {users?.map((user) => (
          <SelectItem key={user.id} value={user.id}>
            {user.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  )
}
