<script setup lang="ts">
import type { RankedChoiceVoteTally } from '@/backend'
import type { PropType } from 'vue'

defineProps({
  options: {
    required: true,
    type: Array as PropType<string[]>
  },
  votes: {
    required: true,
    type: Array as PropType<RankedChoiceVoteTally[]>
  }
})

function get_ranking(vote: number[]) {
  let ranking = new Array<number>(vote.length)
  for (let i = 0; i < vote.length; i++) {
    ranking[vote[i]] = i
  }
  return ranking
}
</script>

<template>
  <table>
    <tr>
      <th>Count</th>
      <th v-for="(_, round) in options" :key="round">No {{ round + 1 }}</th>
    </tr>
    <tr v-for="(ranked_vote, i) in votes" :key="i">
      <td>{{ ranked_vote.count }}</td>
      <td v-for="j in get_ranking(ranked_vote.votes)" :key="j">{{ options[j] }}</td>
    </tr>
  </table>
</template>

<style scoped></style>
