<script setup lang="ts">
import { ref, type PropType } from 'vue';
import { ElectionType, submit_generic_vote, type GenericElection } from '@/backend';
import type { VoteOption } from '@/utils';
import CheckboxChoice from '@/components/voting/CheckboxChoice.vue';

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

interface SntvVoteOption extends VoteOption {
  approve: boolean;
}

const loading = ref<boolean>(false);
const options = ref<SntvVoteOption[]>(props.options.map((option) => {
  return {
    ...option,
    approve: false,
  };
}));


async function submit() {
  const candidates = props.election.options;

  const vote: boolean[] = [];
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const approve = options.value.find(option => option.index == option_index)?.approve as boolean;
    vote.push(approve);
  }

  if (vote.filter(v => v).length > 1) {
    emits(`error`, `You can only vote for one candidate.`);
    return;
  }

  loading.value = true;

  const response = await submit_generic_vote(ElectionType.SingleNonTransferable, props.election.id, props.vote_token, vote);
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
      <CheckboxChoice v-model="options" check_box_title="vote" :single_option="true" />
      <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
    </div>
  </div>
</template>

<style scoped></style>
