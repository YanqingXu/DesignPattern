// 音效接口
interface SoundEffect {
    play(): void;
}

// 背景音乐接口
interface BackgroundMusic {
    play(): void;
}

// 攻击音效
class AttackSound implements SoundEffect {
    play() {
        console.log("Playing attack sound...");
    }
}

// 治疗音效
class HealSound implements SoundEffect {
    play() {
        console.log("Playing heal sound...");
    }
}

// 战斗背景音乐
class BattleMusic implements BackgroundMusic {
    play() {
        console.log("Playing battle music...");
    }
}

// 安静背景音乐
class PeacefulMusic implements BackgroundMusic {
    play() {
        console.log("Playing peaceful music...");
    }
}

// 抽象工厂
interface AudioFactory {
    createSoundEffect(): SoundEffect;
    createBackgroundMusic(): BackgroundMusic;
}

// 战斗工厂
class BattleAudioFactory implements AudioFactory {
    createSoundEffect(): SoundEffect {
        return new AttackSound();
    }

    createBackgroundMusic(): BackgroundMusic {
        return new BattleMusic();
    }
}

// 安静工厂
class PeacefulAudioFactory implements AudioFactory {
    createSoundEffect(): SoundEffect {
        return new HealSound();
    }

    createBackgroundMusic(): BackgroundMusic {
        return new PeacefulMusic();
    }
}

const battleFactory: AudioFactory = new BattleAudioFactory();
const attack = battleFactory.createSoundEffect();
const battleBGM = battleFactory.createBackgroundMusic();

attack.play();
battleBGM.play();

const peacefulFactory: AudioFactory = new PeacefulAudioFactory();
const heal = peacefulFactory.createSoundEffect();
const peacefulBGM = peacefulFactory.createBackgroundMusic();

heal.play();
peacefulBGM.play();
