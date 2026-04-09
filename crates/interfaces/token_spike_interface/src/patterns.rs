//! Паттерны фонем для русского и английского языков

use crate::PhonemeCharacteristics;
use std::collections::HashMap;

/// Создать карту фонем для русского языка
pub fn create_russian_phoneme_map() -> HashMap<char, PhonemeCharacteristics> {
    let mut map = HashMap::new();

    // Гласные (высокие частоты, длинные)
    map.insert('а', PhonemeCharacteristics { phoneme: 'а', frequency: 42.0, duration: 80.0, neuron_id: 0 });
    map.insert('о', PhonemeCharacteristics { phoneme: 'о', frequency: 38.0, duration: 80.0, neuron_id: 1 });
    map.insert('у', PhonemeCharacteristics { phoneme: 'у', frequency: 45.0, duration: 80.0, neuron_id: 2 });
    map.insert('и', PhonemeCharacteristics { phoneme: 'и', frequency: 48.0, duration: 70.0, neuron_id: 3 });
    map.insert('ы', PhonemeCharacteristics { phoneme: 'ы', frequency: 40.0, duration: 70.0, neuron_id: 4 });
    map.insert('э', PhonemeCharacteristics { phoneme: 'э', frequency: 43.0, duration: 75.0, neuron_id: 5 });
    map.insert('е', PhonemeCharacteristics { phoneme: 'е', frequency: 46.0, duration: 75.0, neuron_id: 6 });
    map.insert('ё', PhonemeCharacteristics { phoneme: 'ё', frequency: 44.0, duration: 75.0, neuron_id: 7 });
    map.insert('ю', PhonemeCharacteristics { phoneme: 'ю', frequency: 47.0, duration: 80.0, neuron_id: 8 });
    map.insert('я', PhonemeCharacteristics { phoneme: 'я', frequency: 44.0, duration: 80.0, neuron_id: 9 });

    // Взрывные согласные (короткие, резкие)
    map.insert('п', PhonemeCharacteristics { phoneme: 'п', frequency: 40.0, duration: 40.0, neuron_id: 10 });
    map.insert('б', PhonemeCharacteristics { phoneme: 'б', frequency: 38.0, duration: 40.0, neuron_id: 11 });
    map.insert('т', PhonemeCharacteristics { phoneme: 'т', frequency: 42.0, duration: 35.0, neuron_id: 12 });
    map.insert('д', PhonemeCharacteristics { phoneme: 'д', frequency: 40.0, duration: 35.0, neuron_id: 13 });
    map.insert('к', PhonemeCharacteristics { phoneme: 'к', frequency: 45.0, duration: 40.0, neuron_id: 14 });
    map.insert('г', PhonemeCharacteristics { phoneme: 'г', frequency: 43.0, duration: 40.0, neuron_id: 15 });

    // Фрикативные (шипящие, длинные)
    map.insert('ф', PhonemeCharacteristics { phoneme: 'ф', frequency: 50.0, duration: 60.0, neuron_id: 16 });
    map.insert('в', PhonemeCharacteristics { phoneme: 'в', frequency: 48.0, duration: 60.0, neuron_id: 17 });
    map.insert('с', PhonemeCharacteristics { phoneme: 'с', frequency: 55.0, duration: 65.0, neuron_id: 18 });
    map.insert('з', PhonemeCharacteristics { phoneme: 'з', frequency: 53.0, duration: 65.0, neuron_id: 19 });
    map.insert('ш', PhonemeCharacteristics { phoneme: 'ш', frequency: 52.0, duration: 70.0, neuron_id: 20 });
    map.insert('ж', PhonemeCharacteristics { phoneme: 'ж', frequency: 50.0, duration: 70.0, neuron_id: 21 });
    map.insert('х', PhonemeCharacteristics { phoneme: 'х', frequency: 47.0, duration: 60.0, neuron_id: 22 });

    // Сонорные (звонкие, средние)
    map.insert('м', PhonemeCharacteristics { phoneme: 'м', frequency: 36.0, duration: 55.0, neuron_id: 23 });
    map.insert('н', PhonemeCharacteristics { phoneme: 'н', frequency: 38.0, duration: 55.0, neuron_id: 24 });
    map.insert('л', PhonemeCharacteristics { phoneme: 'л', frequency: 40.0, duration: 50.0, neuron_id: 25 });
    map.insert('р', PhonemeCharacteristics { phoneme: 'р', frequency: 35.0, duration: 50.0, neuron_id: 26 });
    map.insert('й', PhonemeCharacteristics { phoneme: 'й', frequency: 44.0, duration: 45.0, neuron_id: 27 });

    // Аффрикаты
    map.insert('ц', PhonemeCharacteristics { phoneme: 'ц', frequency: 54.0, duration: 50.0, neuron_id: 28 });
    map.insert('ч', PhonemeCharacteristics { phoneme: 'ч', frequency: 51.0, duration: 55.0, neuron_id: 29 });
    map.insert('щ', PhonemeCharacteristics { phoneme: 'щ', frequency: 53.0, duration: 75.0, neuron_id: 30 });

    // Мягкий и твердый знак (пауза)
    map.insert('ь', PhonemeCharacteristics { phoneme: 'ь', frequency: 0.0, duration: 20.0, neuron_id: 31 });
    map.insert('ъ', PhonemeCharacteristics { phoneme: 'ъ', frequency: 0.0, duration: 25.0, neuron_id: 32 });

    map
}

/// Создать карту фонем для английского языка
pub fn create_english_phoneme_map() -> HashMap<char, PhonemeCharacteristics> {
    let mut map = HashMap::new();

    // Гласные
    map.insert('a', PhonemeCharacteristics { phoneme: 'a', frequency: 42.0, duration: 80.0, neuron_id: 0 });
    map.insert('e', PhonemeCharacteristics { phoneme: 'e', frequency: 46.0, duration: 75.0, neuron_id: 1 });
    map.insert('i', PhonemeCharacteristics { phoneme: 'i', frequency: 48.0, duration: 70.0, neuron_id: 2 });
    map.insert('o', PhonemeCharacteristics { phoneme: 'o', frequency: 38.0, duration: 80.0, neuron_id: 3 });
    map.insert('u', PhonemeCharacteristics { phoneme: 'u', frequency: 45.0, duration: 80.0, neuron_id: 4 });

    // Согласные
    map.insert('b', PhonemeCharacteristics { phoneme: 'b', frequency: 38.0, duration: 40.0, neuron_id: 5 });
    map.insert('c', PhonemeCharacteristics { phoneme: 'c', frequency: 45.0, duration: 40.0, neuron_id: 6 });
    map.insert('d', PhonemeCharacteristics { phoneme: 'd', frequency: 40.0, duration: 35.0, neuron_id: 7 });
    map.insert('f', PhonemeCharacteristics { phoneme: 'f', frequency: 50.0, duration: 60.0, neuron_id: 8 });
    map.insert('g', PhonemeCharacteristics { phoneme: 'g', frequency: 43.0, duration: 40.0, neuron_id: 9 });
    map.insert('h', PhonemeCharacteristics { phoneme: 'h', frequency: 47.0, duration: 60.0, neuron_id: 10 });
    map.insert('j', PhonemeCharacteristics { phoneme: 'j', frequency: 44.0, duration: 45.0, neuron_id: 11 });
    map.insert('k', PhonemeCharacteristics { phoneme: 'k', frequency: 45.0, duration: 40.0, neuron_id: 12 });
    map.insert('l', PhonemeCharacteristics { phoneme: 'l', frequency: 40.0, duration: 50.0, neuron_id: 13 });
    map.insert('m', PhonemeCharacteristics { phoneme: 'm', frequency: 36.0, duration: 55.0, neuron_id: 14 });
    map.insert('n', PhonemeCharacteristics { phoneme: 'n', frequency: 38.0, duration: 55.0, neuron_id: 15 });
    map.insert('p', PhonemeCharacteristics { phoneme: 'p', frequency: 40.0, duration: 40.0, neuron_id: 16 });
    map.insert('q', PhonemeCharacteristics { phoneme: 'q', frequency: 45.0, duration: 40.0, neuron_id: 17 });
    map.insert('r', PhonemeCharacteristics { phoneme: 'r', frequency: 35.0, duration: 50.0, neuron_id: 18 });
    map.insert('s', PhonemeCharacteristics { phoneme: 's', frequency: 55.0, duration: 65.0, neuron_id: 19 });
    map.insert('t', PhonemeCharacteristics { phoneme: 't', frequency: 42.0, duration: 35.0, neuron_id: 20 });
    map.insert('v', PhonemeCharacteristics { phoneme: 'v', frequency: 48.0, duration: 60.0, neuron_id: 21 });
    map.insert('w', PhonemeCharacteristics { phoneme: 'w', frequency: 44.0, duration: 55.0, neuron_id: 22 });
    map.insert('x', PhonemeCharacteristics { phoneme: 'x', frequency: 54.0, duration: 50.0, neuron_id: 23 });
    map.insert('y', PhonemeCharacteristics { phoneme: 'y', frequency: 44.0, duration: 70.0, neuron_id: 24 });
    map.insert('z', PhonemeCharacteristics { phoneme: 'z', frequency: 53.0, duration: 65.0, neuron_id: 25 });

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_russian_phoneme_map() {
        let map = create_russian_phoneme_map();
        assert!(map.contains_key(&'а'));
        assert!(map.contains_key(&'п'));
        assert_eq!(map.get(&'а').unwrap().frequency, 42.0);
    }

    #[test]
    fn test_english_phoneme_map() {
        let map = create_english_phoneme_map();
        assert!(map.contains_key(&'a'));
        assert!(map.contains_key(&'b'));
        assert_eq!(map.get(&'a').unwrap().frequency, 42.0);
    }
}
