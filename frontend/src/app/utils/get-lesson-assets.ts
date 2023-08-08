import meta1 from '@/assets/meta/meta1.txt'
import meta2 from '@/assets/meta/meta2.txt'
import meta3 from '@/assets/meta/meta3.txt'
import meta4 from '@/assets/meta/meta4.txt'
import meta5 from '@/assets/meta/meta5.txt'
import meta6 from '@/assets/meta/meta6.txt'
import { useProgramMetadata } from '@/app/hooks/use-metadata'

export function useLessonAssets() {
  return [
    useProgramMetadata(meta1),
    useProgramMetadata(meta2),
    useProgramMetadata(meta3),
    useProgramMetadata(meta4),
    useProgramMetadata(meta5),
    useProgramMetadata(meta6),
  ]
}
