def transformSentence(sentence):
    # write your code here
    wordlist = sentence.split(' ')
    new_sentence = ''
    for word in wordlist:
        for index in range(0, len(word)):
                if index:
                    if word[index] > word[index-1]:
                        new_sentence += word[index].upper()
                    elif word[index] < word[index-1]:
                        new_sentence += word[index].lower()
                    else:
                        new_sentence += new_sentence[-1]
                        continue
                else:
                    new_sentence += word[index]
                    index += 1
                    continue
        new_sentence += ' '
    return new_sentence