<script setup lang="ts">
import type { StvResult, StvRound } from '@/backend'
import { onMounted, ref, type PropType } from 'vue'

const props = defineProps({
  election_result: {
    type: Object as PropType<StvResult>,
    required: true
  }
})

interface Candidate {
  name: String
  won: boolean
}

let candidates = ref<Candidate[]>([])

onMounted(() => {
  candidates.value = props.election_result.candidates.map((candidate, index) => {
    return {
      name: candidate,
      won: props.election_result.elected_candidates.includes(index)
    }
  })
  candidates.value.sort((a, b) => {
    if (a.won && !b.won) {
      return -1
    } else if (!a.won && b.won) {
      return 1
    } else {
      return 0
    }
  })
})
</script>

<template>
  <table>
    <tr>
      <th>Elected</th>
      <th>Name</th>
    </tr>
    <tr v-for="(candidate, i) in candidates" :key="i">
      <td>{{ candidate.won ? '🏆' : '💀' }}</td>
      <td>{{ candidate.name }}</td>
    </tr>
  </table>
</template>

<style scoped></style>
