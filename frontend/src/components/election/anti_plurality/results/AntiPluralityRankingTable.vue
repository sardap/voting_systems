<script setup lang="ts">
import type { AntiPluralityResult } from '@/backend'
import type { PropType } from 'vue'
import { rank_to_emoji } from '@/utils'

defineProps({
  options: {
    type: Object as PropType<string[]>,
    required: true
  },
  result: {
    type: Object as PropType<AntiPluralityResult>,
    required: true
  }
})
</script>

<template>
  <div>
    <p>Remember lower is better</p>
    <table>
      <tr>
        <th>Name</th>
        <th>Votes</th>
        <th>Rank</th>
      </tr>
      <tr v-for="(tally, i) in result.votes_tally" :key="i">
        <td>{{ options[tally.option_index] }}</td>
        <td>
          {{ ((tally.vote_count / result.vote_count) * 100).toFixed(2) }}% ({{ tally.vote_count }})
        </td>
        <td>{{ rank_to_emoji(i + 1) }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped></style>
