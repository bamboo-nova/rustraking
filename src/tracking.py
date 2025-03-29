from ultralytics import YOLO

def run_tracking(video_path: str, model_hash: str, conf_threshold: float):
    # 簡易的なトラッキングコード例
    model = YOLO(model_hash)
    results = model.track(
        source=video_path,
        persist=True,
        verbose=False,
        conf=conf_threshold,
        stream=True,
    )
    
    output = {}
    for result in results:
        if result.boxes is not None:
            for track in result.boxes:
                track_id = int(track.id.cpu().item()) if track.id is not None else -1
                xywhn = track.xywhn.cpu().numpy().tolist()
                xyxy = track.xyxy.cpu().numpy().tolist()
                cls = int(track.cls.cpu().item())
                conf = float(track.conf.cpu().item())
        
                output[track_id] = {
                    "xywhn": xywhn,
                    "xyxy": xyxy,
                    "cls": cls,
                    "conf": conf,
                }
    return output
