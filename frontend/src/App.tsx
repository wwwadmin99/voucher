import { useState } from 'react'
import { ActivationHistory } from '@/components/activation-history'
import { AdminPanel } from '@/components/admin-panel'
import { UserSelect } from '@/components/user-select'
import { VoucherBoard } from '@/components/voucher-board'

function App() {
  const [userId, setUserId] = useState<string | null>(null)

  return (
    <div className="mx-auto flex max-w-4xl flex-col gap-8 p-8">
      <header className="flex flex-col gap-4">
        <h1 className="text-2xl font-semibold">voucher</h1>
        <UserSelect userId={userId} onChange={setUserId} />
      </header>

      {userId && (
        <>
          <section className="flex flex-col gap-4">
            <h2 className="text-lg font-medium">Ваучеры</h2>
            <VoucherBoard userId={userId} />
          </section>

          <section>
            <AdminPanel userId={userId} />
          </section>

          <section className="flex flex-col gap-4">
            <h2 className="text-lg font-medium">История активаций</h2>
            <ActivationHistory userId={userId} />
          </section>
        </>
      )}
    </div>
  )
}

export default App
