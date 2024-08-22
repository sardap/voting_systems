<script setup lang="ts">
import type { ThreeTwoOneResult } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  result: {
    type: Object as PropType<ThreeTwoOneResult>,
    required: true
  }
})

function vote_to_string(vote: number): string {
  if (vote == 0) {
    return '😞 Bad'
  } else if (vote == 1) {
    return '🤔 Ok'
  }
  return '😀 Good'
}
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Count</th>
        <th v-for="name in result.options" :key="name">{{ name }}</th>
      </tr>
      <tr v-for="(tally, i) in result.vote_tally" :key="i">
        <td>{{ tally.count }}</td>
        <td v-for="(vote, j) in tally.votes" :key="j">{{ vote_to_string(vote) }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped></style>
