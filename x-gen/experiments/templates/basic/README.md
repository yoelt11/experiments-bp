# [Experiment Name]

## About

[Provide a brief overview of the experiment, its purpose, and significance]

## Question

What question does this experiment want to answer?

[Clearly state the primary research question or hypothesis being tested]

## Expected Results

[Describe what results you expect to see and why, including any quantitative expectations]

## Concrete Implementation Plan

[Detail the specific steps you will take to conduct this experiment, including:]
- Data preparation steps
- Model architecture or methodology
- Evaluation metrics
- Timeline and milestones

## Directory Structure

```
├── README.md              # This file - experiment documentation and usage notes
├── common.py              # Shared utilities (helpers, configs, reusable functions)
├── run_training.py        # Main training entry point
├── evaluate.py            # Model evaluation / metrics computation
├── run_pipeline.py        # End-to-end pipeline (training + evaluation)
├── results/               # Stores training outputs / checkpoints
│   └── train_results.pkl
└── figures/               # Generated plots / visualizations
    ├── rl2_curves.png
    ├── loss_curves.png
    ├── gradient_ratios.png
    └── summary_table.txt
```

## Usage

[Instructions on how to run the experiment, including any prerequisites and commands]

## Results

[Document the actual results obtained from the experiment]

## Conclusion

[Summarize the findings and their implications]