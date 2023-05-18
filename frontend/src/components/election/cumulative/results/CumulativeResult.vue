<script setup lang="ts">
import type { GenericElection, CumulativeResult, GenericElectionResult } from '@/backend';
import { ref, type PropType } from 'vue';
import CumulativeTable from './CumulativeTable.vue';

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

const result = ref(props.result as CumulativeResult)

function score_sum() {
  return result.value?.votes_tally.reduce((a, b) => a + b.vote_count, 0);
}

</script>

<template>
  <h2>Score Sum: {{ score_sum() }}</h2>
  <br />
  <h2>Ranking Table</h2>
  <CumulativeTable :result="result" />
</template>

<style scoped></style>
