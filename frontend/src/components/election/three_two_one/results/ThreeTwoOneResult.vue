<script setup lang="ts">
import type { GenericElection, ThreeTwoOneResult, GenericElectionResult } from '@/backend';
import { ref, type PropType } from 'vue';
import ThreeTwoOnePoints from './ThreeTwoOnePoints.vue';
import ThreeTwoOneSemifinalist from './ThreeTwoOneSemifinalist.vue'
import ThreeTwoOneFinalist from './ThreeTwoOneFinalist.vue';
import ThreeTwoOneVoteTally from './ThreeTwoOneVoteTally.vue';

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

const result = ref(props.result as ThreeTwoOneResult);

</script>

<template>
  <h2>Points Table</h2>
  <ThreeTwoOnePoints :result="result" />
  <h2>Semifinalist</h2>
  <ThreeTwoOneSemifinalist :result="result" />
  <h2>Finalist</h2>
  <ThreeTwoOneFinalist :result="result" />
  <h2>Votes</h2>
  <ThreeTwoOneVoteTally :result="result" />
</template>

<style scoped></style>
