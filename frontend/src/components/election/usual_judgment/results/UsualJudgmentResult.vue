<script setup lang="ts">
import type { GenericElection, GenericElectionResult, UsualJudgmentResult } from '@/backend';
import { ref, type PropType } from 'vue';
import UsualJudgmentTally from './UsualJudgmentTally.vue';
import UsualJudgmentScores from './UsualJudgmentScores.vue';

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true,
  },
  result: {
    type: Object as PropType<GenericElectionResult>,
    required: true,
  }
});

const result = ref(props.result as UsualJudgmentResult)

</script>

<template>
  <div>
    <h2>Winner: {{ result.options[result.winner] }} 🥇</h2>
    <br />
    <h2>Tally</h2>
    <UsualJudgmentTally :options="result.options.map((name, i) => { return { option_index: i, name: name } })"
      :tally="result.starting_tally" :vote_count="result.vote_count" />
    <br />
    <div v-if="result.tie_info">
      <h2>Tie Found</h2>
      <UsualJudgmentScores
        :scores="result.tie_info.scores.map((i) => { return { name: election?.options[i.option_index] as string, score: i.score } }).sort((a, b) => { return b.score - a.score })" />
    </div>
  </div>
</template>

<style scoped></style>
