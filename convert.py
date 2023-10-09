import os

import torch
from onnxruntime.transformers import optimizer
from torch import nn
from transformers import AutoModel


class EmbeddingModel(nn.Module):
    def __init__(self):
        super().__init__()

        self.backbone = AutoModel.from_pretrained('BAAI/bge-large-en-v1.5')

    def forward(
        self,
        input_ids: torch.Tensor,
        attention_mask: torch.Tensor,
        token_type_ids: torch.Tensor,
    ) -> torch.Tensor:
        return self.backbone(input_ids, attention_mask, token_type_ids, return_dict=True).last_hidden_state[:, 0]


def main():
    model = EmbeddingModel()
    model.eval()

    torch.onnx.export(
        model,
        (
            torch.zeros((1, 512), dtype=torch.long),
            torch.zeros((1, 512), dtype=torch.long),
            torch.zeros((1, 512), dtype=torch.long),
        ),
        './model_repository/embedding/1/v1.onnx',
        export_params=True,
        opset_version=16,
        do_constant_folding=True,
        input_names=['input_ids', 'attention_mask', 'token_type_ids'],
        output_names=['embedding'],
        dynamic_axes={
            'input_ids': {0: 'batch_size', 1: 'sequence'},
            'attention_mask': {0: 'batch_size', 1: 'sequence'},
            'token_type_ids': {0: 'batch_size', 1: 'sequence'},
            'embedding': {0: 'batch_size'},
        },
    )

    optimized_model = optimizer.optimize_model(
        './model_repository/embedding/1/v1.onnx',
        model_type='bert',
        num_heads=16,
        hidden_size=1024,
        opt_level=99,
        use_gpu=True,
    )
    optimized_model.save_model_to_file('./model_repository/embedding/1/v1.onnx')


if __name__ == '__main__':
    root_path: str = './model_repository'
    for module in ('embedding', 'model', 'tokenizer'):
        os.makedirs(f'{root_path}/{module}/1', exist_ok=True)

    main()
