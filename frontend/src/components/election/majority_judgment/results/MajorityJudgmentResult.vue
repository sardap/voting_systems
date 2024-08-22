<script setup lang="ts">
import type { GenericElection, MajorityJudgmentResult, GenericElectionResult } from '@/backend'
import { ref, type PropType } from 'vue'
import MajorityJudgmentTally from './MajorityJudgmentTally.vue'
import ScoreCount from '@/components/results/ScoreCount.vue'

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true
  },
  result: {
    type: Object as PropType<GenericElectionResult>,
    required: true
  }
})

const result = ref(props.result as MajorityJudgmentResult)
</script>

<template>
  <h2>Winner: {{ result.options[result.winner] }} 🥇</h2>
  <br />
  <h2>Starting Tally</h2>
  <MajorityJudgmentTally
    :options="
      result.options.map((name, i) => {
        return { option_index: i, name: name }
      })
    "
    :tally="result.starting_tally"
    :vote_count="result.vote_count"
  />
  <br />
  <h2>Modified Tally</h2>
  <MajorityJudgmentTally
    :options="result.runoff.participants.map((i) => { return { option_index: i, name: result?.options[i] as string } })"
    :tally="result.runoff.modified_tally"
    :vote_count="result.vote_count"
  />
  <div v-if="result.score_result">
    <h2>Since another tie deciding winner based on score</h2>
    <ScoreCount
      :options="result.options"
      :counts="result.score_result.vote_tally"
      :winner="result.winner"
    />
  </div>
</template>

<style scoped></style>
