import { createRouter, createMemoryHistory } from "vue-router";
import WizardLayout from "@/components/WizardLayout.vue";
import ChooseEdition from "@/components/steps/ChooseEdition.vue";
import ConfigureEdition from "@/components/steps/ConfigureEdition.vue";
import CreateSDCard from "@/components/steps/CreateSDCard.vue";

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: "/",
      components: {
        default: WizardLayout,
      },
      children: [
        {
          path: "",
          name: "choose-edition",
          components: {
            default: ChooseEdition,
          },
        },
        {
          path: "configure/:edition",
          name: "configure-edition",
          components: {
            default: ConfigureEdition,
          },
        },
        {
          path: "sd-card",
          name: "sd-card",
          components: {
            default: CreateSDCard,
          },
        },
      ],
    },
  ],
});

export default router;
