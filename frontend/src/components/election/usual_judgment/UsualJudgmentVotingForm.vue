<script setup lang="ts">
import { ref, type PropType } from 'vue';
import { from_mj_rating_to_string, ElectionType, submit_generic_vote, type GenericElection, MJRating, from_mj_rating_to_int, ujgrade_to_string, ujgrade_to_number, UJGrade } from '@/backend';
import type { VoteOption } from '@/utils';

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
const options = ref(props.options.map((option) => {
  return {
    ...option,
    grade: UJGrade.Bad,
  };
}));
const grades: UJGrade[] = [UJGrade.Bad, UJGrade.Inadequate, UJGrade.Passable, UJGrade.Fair, UJGrade.Good, UJGrade.VeryGood, UJGrade.Excellent];


async function submit() {
  const candidates = props.election.options;

  const vote: number[] = [];
  for (let option_index = 0; option_index < candidates.length; option_index++) {
    const points = ujgrade_to_number(options.value.find(option => option.index == option_index)?.grade as UJGrade);
    vote.push(points);
  }

  loading.value = true;

  const response = await submit_generic_vote(ElectionType.UsualJudgment, props.election.id, props.vote_token, vote);
  const data = await response.text();
  loading.value = false;
  if (!response.ok) {
    emits(`error`, data);
    console.log(`ERROR: ${data}`);
    return;
  }

  emits(`complete`);
}

function change_rating(i: number, grade: UJGrade) {
  options.value[i].grade = grade;
}

function get_class(i: number, grade: UJGrade) {
  if (options.value[i].grade != grade) {
    return "";
  }

  switch (grade) {
    case UJGrade.Bad:
      return "bad";
    case UJGrade.Inadequate:
      return "inadequate";
    case UJGrade.Passable:
      return "passable";
    case UJGrade.Fair:
      return "fair";
    case UJGrade.Good:
      return "good";
    case UJGrade.VeryGood:
      return "very-good";
    case UJGrade.Excellent:
      return "excellent";
  }
}

</script>

<template>
  <div>
    <div v-if="loading">
      <p>Loading...</p>
    </div>
    <div>
      <div class="table">
        <div class="row header">
          <div class="col">
            <p>Name</p>
          </div>
          <div class="col">
            <p>Rating</p>
          </div>
        </div>
        <div v-for="(option, i) in options" :key="option.index" class="row">
          <div class="col">
            <p>{{ option.name }}</p>
          </div>
          <div class="col">
            <button v-for="rating in grades" :class="`rect-button ` + (get_class(i, rating))"
              @click="change_rating(i, rating)">{{ ujgrade_to_string(rating) }}</button>
          </div>
        </div>
      </div>
    </div>
    <button class="vote-button" @click="submit">🗳️VOTE🗳️</button>
  </div>
</template>

<style scoped>
@import '../../../assets/voting_table.css';

/* General styles for the circle buttons */
.rect-button {
  display: inline-block;
  width: 90px;
  height: 30px;
  font-size: small;
  text-align: center;
  font-weight: bold;
  background-color: white;
  cursor: pointer;
  user-select: none;
  margin: 0 5px;
  margin-top: 3px;
  margin-bottom: 3px;
}

.rect-button:hover {
  opacity: 0.8;
}

.rect-button.bad {
  background-color: #8b0000;
  /* Dark red color for 'Bad' */
  color: #ffffff;
}

.rect-button.inadequate {
  background-color: #ff4500;
  /* Orange-red color for 'Inadequate' */
  color: #ffffff;
}

.rect-button.passable {
  background-color: #ffa500;
  /* Orange color for 'Passable' */
  color: #ffffff;
}

.rect-button.fair {
  background-color: #ffd700;
  /* Gold color for 'Fair' */
  color: #ffffff;
}

.rect-button.good {
  background-color: #9acd32;
  /* Yellow-green color for 'Good' */
  color: #ffffff;
}

.rect-button.very-good {
  background-color: #008000;
  /* Green color for 'Very Good' */
  color: #ffffff;
}

.rect-button.excellent {
  background-color: #006400;
  /* Dark green color for 'Excellent' */
  color: #ffffff;
}
</style>
