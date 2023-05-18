<script setup lang="ts">
import { ref, type PropType } from 'vue';
import { type GenericElection, submit_generic_vote, ElectionType } from '@/backend';
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
  const votes: number[] = [];
  for (let i = 0; i < options.value.length; i++) {
    votes.push(options.value.findIndex((option) => option.index === i));
  }

  loading.value = true;
  const response = await submit_generic_vote(ElectionType.PreferentialVoting, props.election.id, props.vote_token, votes);
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
    <div>
      <RankedChoice v-model="options" />
      <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
    </div>
  </div>
</template>

<style scoped></style>
