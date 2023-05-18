<script setup lang="ts">
import { ref, type PropType } from 'vue';
import { ElectionType, submit_generic_vote, type GenericElection } from '@/backend';
import type { VoteOption } from '@/utils';
import RankedChoice from '@/components/voting/RankedChoice.vue';

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true,
  },
  options: {
    type: Array as PropType<VoteOption[]>,
    required: true,
  },
  vote_token: {
    type: String,
    required: false,
  }
});

const emits = defineEmits({
  complete: () => true,
  error: (error: string) => true,
});


const loading = ref<boolean>(false);
const options = ref<VoteOption[]>(props.options);


async function submit() {
  const candidates = props.election.options;

  const vote = [];
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const vote_count = options.value.length - options.value.findIndex(option => option.index == option_index) - 1;
    console.log(`${candidates[option_index]}: ${vote_count}`)
    vote.push(vote_count);
  }

  loading.value = true;
  const response = await submit_generic_vote(ElectionType.BordaCount, props.election.id, props.vote_token, vote);
  const data = await response.text();
  loading.value = false;
  if (!response.ok) {
    emits(`error`, data);
    return;
  }

  emits(`complete`);
}
</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <RankedChoice v-if="election" v-model="options" :reverse_order="true" rank_title="Points" />
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped></style>
