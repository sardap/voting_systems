<script setup lang="ts">
import type { GenericElection, ScoreResult, GenericElectionResult } from '@/backend';
import { ref, type PropType } from 'vue';
import ScoreCount from '@/components/results/ScoreCount.vue';

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

const result = ref(props.result as ScoreResult);
</script>

<template>
  <h2>Winner: {{ result.options[result.winner] }} 🥇</h2>
  <h2>Score Count</h2>
  <ScoreCount :options="result.options" :counts="result.vote_tally" :winner="result.winner" />
</template>

<style scoped></style>
