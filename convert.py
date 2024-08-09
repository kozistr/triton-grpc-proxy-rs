import os

import torch
from onnxruntime.transformers import optimizer
from torch import nn
from torch.nn.functional import normalize
from transformers import AutoModel


class EmbeddingModel(nn.Module):
    def __init__(self, model_name: str = 'BAAI/bge-m3'):
        super().__init__()

        self.backbone = AutoModel.from_pretrained(model_name)

    def forward(self, input_ids: torch.Tensor, attention_mask: torch.Tensor) -> torch.Tensor:
        return normalize(
            self.backbone(input_ids, attention_mask, return_dict=True).last_hidden_state[:, 0], 
            p=2.0, 
            dim=1,
        )


def main():
    model = EmbeddingModel()
    model.eval()

    torch.onnx.export(
        model,
        (
            torch.zeros((1, 8192), dtype=torch.long),
            torch.zeros((1, 8192), dtype=torch.long),
        ),
        './model_repository/embedding/1/model.onnx',
        export_params=True,
        opset_version=17,
        do_constant_folding=True,
        input_names=['input_ids', 'attention_mask'],
        output_names=['embedding'],
        dynamic_axes={
            'input_ids': {0: 'batch_size', 1: 'sequence'},
            'attention_mask': {0: 'batch_size', 1: 'sequence'},
            'embedding': {0: 'batch_size'},
        },
    )

    try:
        optimized_model = optimizer.optimize_model(
            './model_repository/embedding/1/model.onnx',
            model_type='bert',
            num_heads=16,
            hidden_size=1024,
            opt_level=99,
            use_gpu=True,
        )
        optimized_model.save_model_to_file('./model_repository/embedding/1/model.onnx')
    except Exception:
        print('failed to optimize the onnx model')


if __name__ == '__main__':
    root_path: str = './model_repository'
    for module in ('embedding', 'model', 'tokenizer'):
        os.makedirs(f'{root_path}/{module}/1', exist_ok=True)

    main()
