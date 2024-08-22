<script setup lang="ts">
import type { StarResult } from '@/backend'
import type { PropType } from 'vue'
import { rank_to_emoji } from '@/utils'

defineProps({
  options: {
    type: Object as PropType<string[]>,
    required: true
  },
  result: {
    type: Object as PropType<StarResult>,
    required: true
  }
})
</script>

<template>
  <div>
    <table>
      <tr>
        <th>Name</th>
        <th>Votes</th>
        <th>Rank</th>
      </tr>
      <tr v-for="(score, i) in result.runoff" :key="i">
        <td>{{ options[score.option_index] }}</td>
        <td>
          {{ ((score.vote_count / result.vote_count) * 100).toFixed(2) }}% ({{ score.vote_count }})
        </td>
        <td>{{ rank_to_emoji(i + 1) }}</td>
      </tr>
    </table>
  </div>
</template>

<style scoped></style>
